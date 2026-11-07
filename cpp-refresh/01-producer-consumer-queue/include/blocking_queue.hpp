#ifndef PCQ_BLOCKING_QUEUE_HPP
#define PCQ_BLOCKING_QUEUE_HPP

// Thread-safe, optionally bounded, multi-producer / multi-consumer queue.
//
// Design goals (Phase 1 capstone):
//   - Works with move-only element types (e.g. std::unique_ptr<T>).
//   - No element is ever copied: push takes an rvalue and moves it in,
//     pop moves it out.
//   - close() lets producers signal "no more items"; blocked consumers wake
//     and drain what's left, then see the queue as finished.
//   - C++11 only: std::mutex, std::condition_variable, std::unique_lock.

#include <condition_variable>
#include <cstddef>
#include <deque>
#include <mutex>

namespace pcq {

template <typename T>
class BlockingQueue {
public:
    // capacity == 0 means unbounded.
    explicit BlockingQueue(std::size_t capacity = 0) : capacity_(capacity) {}

    // A queue owns a mutex and condition variables: neither copyable nor
    // movable.
    BlockingQueue(const BlockingQueue&) = delete;
    BlockingQueue& operator=(const BlockingQueue&) = delete;

    // Blocks while the queue is full. Returns false if the queue is closed
    // (the item is not enqueued).
    bool push(T&& item);

    // Non-blocking variant. Returns false if full or closed.
    bool try_push(T&& item);

    // Blocks until an item is available or the queue is closed and empty.
    // On success moves the item into `out` and returns true.
    bool pop(T& out);

    // Non-blocking variant. Returns false if empty; after close() it still
    // drains remaining items.
    bool try_pop(T& out);

    // After close(): pushes fail, pops drain remaining items then fail.
    void close();

    bool closed() const;
    std::size_t size() const;

private:
    mutable std::mutex mutex_;
    std::condition_variable not_empty_;
    std::condition_variable not_full_;
    std::deque<T> items_;
    std::size_t capacity_;
    bool closed_ = false;
};

template <typename T>
bool BlockingQueue<T>::push(T&& item) {
    {
        std::unique_lock<std::mutex> lock(mutex_);
        not_full_.wait(lock, [&]{ return closed_ || capacity_ == 0 || items_.size() < capacity_; });
        if (closed_) return false;
        items_.push_back(std::move(item));
    }
    not_empty_.notify_one();
    return true;
}

template <typename T>
bool BlockingQueue<T>::try_push(T&& item) {
    {
        std::lock_guard<std::mutex> lock(mutex_);
        if (closed_) return false;
        if (capacity_ != 0 && items_.size() >= capacity_) return false;
        items_.push_back(std::move(item));
    }
    not_empty_.notify_one();
    return true;
}

template <typename T>
bool BlockingQueue<T>::pop(T& out) {
    {
        std::unique_lock<std::mutex> lock(mutex_);
        not_empty_.wait(lock, [&]{ return closed_ || !items_.empty(); });
        if (items_.empty()) return false;
        out = std::move(items_.front());
        items_.pop_front();
    }
    not_full_.notify_one();
    return true;
}

template <typename T>
bool BlockingQueue<T>::try_pop(T& out) {
    {
        std::lock_guard<std::mutex> lock(mutex_);
        // Like pop(), a closed queue still yields its remaining items.
        if (items_.empty()) return false;
        out = std::move(items_.front());
        items_.pop_front();
    }
    not_full_.notify_one();
    return true;
}

template <typename T>
void BlockingQueue<T>::close() {
    {
        std::lock_guard<std::mutex> lock(mutex_);
        closed_ = true;
    }
    not_empty_.notify_all();
    not_full_.notify_all();
}

template <typename T>
bool BlockingQueue<T>::closed() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return closed_;
}

template <typename T>
std::size_t BlockingQueue<T>::size() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return items_.size();
}

}  // namespace pcq

#endif  // PCQ_BLOCKING_QUEUE_HPP
