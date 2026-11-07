#include <blocking_queue.hpp>

#include <gtest/gtest.h>

#include <atomic>
#include <memory>
#include <thread>
#include <type_traits>

namespace {

using IntPtr = std::unique_ptr<int>;

// Compile-time checks that pass with the scaffold.
static_assert(!std::is_copy_constructible<pcq::BlockingQueue<int>>::value,
              "queue must not be copyable");
static_assert(!std::is_copy_constructible<IntPtr>::value,
              "test element type must be move-only");

TEST(BlockingQueue, InstantiatesWithMoveOnlyType) {
    pcq::BlockingQueue<IntPtr> q(4);
    EXPECT_EQ(q.size(), 0u);
    EXPECT_FALSE(q.closed());
}

// Behaviour tests for the capstone. Remove the DISABLED_ prefix as each
// piece is implemented.

TEST(BlockingQueue, PushThenPopTransfersOwnership) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(42))));
    IntPtr out;
    ASSERT_TRUE(q.pop(out));
    ASSERT_NE(out, nullptr);
    EXPECT_EQ(*out, 42);
}

TEST(BlockingQueue, PreservesFifoOrder) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(42))));
    ASSERT_TRUE(q.push(IntPtr(new int(69))));
    IntPtr out;
    ASSERT_TRUE(q.pop(out));
    ASSERT_NE(out, nullptr);
    EXPECT_EQ(*out, 42);
    ASSERT_TRUE(q.pop(out));
    ASSERT_NE(out, nullptr);
    EXPECT_EQ(*out, 69);
}

TEST(BlockingQueue, TryPushFailsWhenFull) {
    pcq::BlockingQueue<IntPtr> q(3);
    ASSERT_TRUE(q.try_push(IntPtr(new int(1))));
    ASSERT_TRUE(q.try_push(IntPtr(new int(2))));
    ASSERT_TRUE(q.try_push(IntPtr(new int(3))));
    ASSERT_FALSE(q.try_push(IntPtr(new int(4))));
}

TEST(BlockingQueue, TryPopFailsWhenEmpty) {
    pcq::BlockingQueue<IntPtr> q;
    IntPtr out;
    ASSERT_FALSE(q.try_pop(out));
}

TEST(BlockingQueue, PushFailsAfterClose) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(1))));
    q.close();
    ASSERT_FALSE(q.push(IntPtr(new int(2))));
}

TEST(BlockingQueue, CloseWakesBlockedConsumer) {
    pcq::BlockingQueue<IntPtr> q;
    bool pop_result = true;

    std::thread consumer([&] {
        IntPtr out;
        pop_result = q.pop(out);
    });

    q.close();
    consumer.join();

    EXPECT_FALSE(pop_result);
}

TEST(BlockingQueue, ConsumersDrainRemainingItemsAfterClose) {
    pcq::BlockingQueue<IntPtr> q;
    const int N = 10;

    for (int i = 0; i < N; ++i) {
        ASSERT_TRUE(q.push(IntPtr(new int(i))));
    }
    q.close();

    std::atomic<int> drained{0};
    auto drain = [&] {
        IntPtr out;
        while (q.pop(out)) {
            ++drained;
        }
    };

    std::thread t1(drain);
    std::thread t2(drain);
    t1.join();
    t2.join();

    EXPECT_EQ(drained.load(), N);
}

TEST(BlockingQueue, ManyProducersManyConsumersDeliverEveryItemOnce) {
    // Run under the tsan preset to catch data races.
    const int P = 4;   // producers
    const int C = 4;   // consumers
    const int K = 25;  // items per producer; total = P * K = 100

    pcq::BlockingQueue<int> q;

    // counts[i] must be exactly 1 after the test; use atomic to avoid a
    // data race in the consumer threads.
    const std::size_t total = static_cast<std::size_t>(P * K);
    std::vector<std::atomic<int>> counts(total);
    for (auto& cnt : counts) cnt.store(0);

    // Each producer owns a unique range [p*K, (p+1)*K).
    std::vector<std::thread> producers;
    for (int p = 0; p < P; ++p) {
        producers.emplace_back([&q, p] {
            for (int i = p * K; i < (p + 1) * K; ++i) {
                q.push(std::move(i));
            }
        });
    }

    std::vector<std::thread> consumers;
    for (int c = 0; c < C; ++c) {
        consumers.emplace_back([&q, &counts] {
            int item = 0;
            while (q.pop(item)) {
                ++counts[static_cast<std::size_t>(item)];
            }
        });
    }

    for (auto& t : producers) t.join();
    q.close();
    for (auto& t : consumers) t.join();

    for (std::size_t i = 0; i < total; ++i) {
        EXPECT_EQ(counts[i].load(), 1) << "item " << i;
    }
}

}  // namespace
