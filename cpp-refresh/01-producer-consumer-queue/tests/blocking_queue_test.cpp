#include <blocking_queue.hpp>

#include <gtest/gtest.h>

#include <atomic>
#include <chrono>
#include <memory>
#include <thread>
#include <type_traits>
#include <vector>

namespace {

using IntPtr = std::unique_ptr<int>;

// Compile-time checks on the queue's type properties.
static_assert(!std::is_copy_constructible<pcq::BlockingQueue<int>>::value,
              "queue must not be copyable");
static_assert(!std::is_copy_constructible<IntPtr>::value,
              "test element type must be move-only");

TEST(BlockingQueue, InstantiatesWithMoveOnlyType) {
    pcq::BlockingQueue<IntPtr> q(4);
    EXPECT_EQ(q.size(), 0u);
    EXPECT_FALSE(q.closed());
}

// Behaviour tests. Run the concurrency tests under the tsan preset.

// Long enough for a thread that is going to block to reach its wait.
// Used only to assert that something has NOT happened yet, so a slow
// machine can make these tests weaker but never makes them fail spuriously.
const std::chrono::milliseconds kSettle(50);

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

TEST(BlockingQueue, TryPopReturnsItemWhenNonEmpty) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(42))));
    IntPtr out;
    ASSERT_TRUE(q.try_pop(out));
    ASSERT_NE(out, nullptr);
    EXPECT_EQ(*out, 42);
    EXPECT_EQ(q.size(), 0u);
    EXPECT_FALSE(q.try_pop(out));
}

TEST(BlockingQueue, TryPopDrainsRemainingItemsAfterClose) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(1))));
    ASSERT_TRUE(q.push(IntPtr(new int(2))));
    q.close();

    IntPtr out;
    ASSERT_TRUE(q.try_pop(out));
    EXPECT_EQ(*out, 1);
    ASSERT_TRUE(q.try_pop(out));
    EXPECT_EQ(*out, 2);
    EXPECT_FALSE(q.try_pop(out));
}

TEST(BlockingQueue, TryPopFreesSlotForBlockedProducer) {
    pcq::BlockingQueue<IntPtr> q(1);
    ASSERT_TRUE(q.push(IntPtr(new int(1))));

    std::thread producer([&] { EXPECT_TRUE(q.push(IntPtr(new int(2)))); });

    // try_pop must notify not_full_, or the producer never wakes and
    // join() hangs.
    IntPtr out;
    ASSERT_TRUE(q.try_pop(out));
    EXPECT_EQ(*out, 1);
    producer.join();

    ASSERT_TRUE(q.pop(out));
    EXPECT_EQ(*out, 2);
}

TEST(BlockingQueue, PushFailsAfterClose) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(1))));
    q.close();
    ASSERT_FALSE(q.push(IntPtr(new int(2))));
}

TEST(BlockingQueue, CloseWakesBlockedConsumer) {
    pcq::BlockingQueue<IntPtr> q;
    std::atomic<bool> returned{false};
    bool pop_result = true;

    std::thread consumer([&] {
        IntPtr out;
        pop_result = q.pop(out);
        returned = true;
    });

    // Give the consumer time to block in pop() so close() has to wake it,
    // rather than pop() seeing an already-closed queue.
    std::this_thread::sleep_for(kSettle);
    EXPECT_FALSE(returned.load()) << "pop must block while the queue is empty";

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

TEST(BlockingQueue, PushBlocksWhenFullUntilConsumerPops) {
    pcq::BlockingQueue<IntPtr> q(1);
    ASSERT_TRUE(q.push(IntPtr(new int(1))));

    std::atomic<bool> pushed{false};
    std::thread producer([&] {
        EXPECT_TRUE(q.push(IntPtr(new int(2))));
        pushed = true;
    });

    std::this_thread::sleep_for(kSettle);
    EXPECT_FALSE(pushed.load()) << "push must block while the queue is full";

    IntPtr out;
    ASSERT_TRUE(q.pop(out));
    EXPECT_EQ(*out, 1);

    producer.join();
    EXPECT_TRUE(pushed.load());
    ASSERT_TRUE(q.pop(out));
    EXPECT_EQ(*out, 2);
}

TEST(BlockingQueue, CloseWakesBlockedProducer) {
    pcq::BlockingQueue<IntPtr> q(1);
    ASSERT_TRUE(q.push(IntPtr(new int(1))));

    std::atomic<bool> returned{false};
    bool push_result = true;
    std::thread producer([&] {
        push_result = q.push(IntPtr(new int(2)));
        returned = true;
    });

    std::this_thread::sleep_for(kSettle);
    EXPECT_FALSE(returned.load()) << "push must block while the queue is full";

    q.close();
    producer.join();

    EXPECT_FALSE(push_result) << "push woken by close() must fail";
    EXPECT_EQ(q.size(), 1u) << "rejected item must not be enqueued";

    // The item enqueued before close() is still delivered.
    IntPtr out;
    ASSERT_TRUE(q.pop(out));
    EXPECT_EQ(*out, 1);
    EXPECT_FALSE(q.pop(out));
}

TEST(BlockingQueue, ObserversAreSafeDuringConcurrentUse) {
    // Regression test for size()/closed() reading without the lock. The
    // values are inherently racy snapshots; the point is that tsan reports
    // no data race while another thread mutates the queue.
    const int N = 1000;
    pcq::BlockingQueue<int> q;

    std::thread producer([&q] {
        for (int i = 0; i < N; ++i) q.push(int(i));
        q.close();
    });

    std::size_t max_seen = 0;
    while (!q.closed()) {
        std::size_t s = q.size();
        if (s > max_seen) max_seen = s;
    }
    producer.join();

    EXPECT_LE(max_seen, static_cast<std::size_t>(N));
    EXPECT_EQ(q.size(), static_cast<std::size_t>(N));
}

// Runs P producers and C consumers through a queue of the given capacity
// and checks every item is delivered exactly once. Run under the tsan
// preset to catch data races.
void run_mpmc(std::size_t capacity, int P, int C, int K) {
    pcq::BlockingQueue<int> q(capacity);

    // counts[i] must be exactly 1 after the run; atomic because consumer
    // threads increment them concurrently.
    const std::size_t total = static_cast<std::size_t>(P * K);
    std::vector<std::atomic<int>> counts(total);
    for (auto& cnt : counts) cnt.store(0);

    std::atomic<int> failed_pushes{0};

    // Each producer owns a unique range [p*K, (p+1)*K).
    std::vector<std::thread> producers;
    for (int p = 0; p < P; ++p) {
        producers.emplace_back([&q, &failed_pushes, p, K] {
            for (int i = p * K; i < (p + 1) * K; ++i) {
                if (!q.push(int(i))) ++failed_pushes;
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

    EXPECT_EQ(failed_pushes.load(), 0);
    for (std::size_t i = 0; i < total; ++i) {
        EXPECT_EQ(counts[i].load(), 1) << "item " << i;
    }
}

TEST(BlockingQueue, ManyProducersManyConsumersDeliverEveryItemOnce) {
    run_mpmc(/*capacity=*/0, /*P=*/4, /*C=*/4, /*K=*/25);
}

TEST(BlockingQueue, ManyProducersManyConsumersBoundedUnderContention) {
    // A tiny capacity forces producers to block on not_full_ and consumers
    // on not_empty_ repeatedly, exercising both wake-up paths.
    run_mpmc(/*capacity=*/2, /*P=*/4, /*C=*/4, /*K=*/1000);
}

}  // namespace
