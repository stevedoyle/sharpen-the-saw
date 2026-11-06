#include <blocking_queue.hpp>

#include <gtest/gtest.h>

#include <memory>
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

TEST(BlockingQueue, DISABLED_PushThenPopTransfersOwnership) {
    pcq::BlockingQueue<IntPtr> q;
    ASSERT_TRUE(q.push(IntPtr(new int(42))));
    IntPtr out;
    ASSERT_TRUE(q.pop(out));
    ASSERT_NE(out, nullptr);
    EXPECT_EQ(*out, 42);
}

TEST(BlockingQueue, DISABLED_PreservesFifoOrder) {
    // TODO(capstone)
}

TEST(BlockingQueue, DISABLED_TryPushFailsWhenFull) {
    // TODO(capstone)
}

TEST(BlockingQueue, DISABLED_TryPopFailsWhenEmpty) {
    // TODO(capstone)
}

TEST(BlockingQueue, DISABLED_PushFailsAfterClose) {
    // TODO(capstone)
}

TEST(BlockingQueue, DISABLED_CloseWakesBlockedConsumer) {
    // TODO(capstone): a consumer blocked in pop() returns false after close().
}

TEST(BlockingQueue, DISABLED_ConsumersDrainRemainingItemsAfterClose) {
    // TODO(capstone)
}

TEST(BlockingQueue, DISABLED_ManyProducersManyConsumersDeliverEveryItemOnce) {
    // TODO(capstone): run under the tsan preset.
}

}  // namespace
