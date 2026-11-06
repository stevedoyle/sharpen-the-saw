#ifndef PCQ_BLOCKING_QUEUE_HPP
#define PCQ_BLOCKING_QUEUE_HPP

// Thread-safe, optionally bounded, multi-producer / multi-consumer queue.
//
// Design goals (Phase 1 capstone):
//   - Works with move-only element types (e.g. std::unique_ptr<T>).
//   - No element is ever copied: push takes by value/rvalue and moves in,
//     pop moves out.
//   - close() lets producers signal "no more items"; blocked consumers wake
//     and drain what's left, then see the queue as finished.
//   - C++11 only: std::mutex, std::condition_variable, std::unique_lock.
//
// This file is a scaffold: the interface is sketched, the bodies are yours.

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
    bool push(T item);

    // Non-blocking variant. Returns false if full or closed.
    bool try_push(T item);

    // Blocks until an item is available or the queue is closed and empty.
    // On success moves the item into `out` and returns true.
    bool pop(T& out);

    // Non-blocking variant. Returns false if empty.
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

// TODO(capstone): implement the member functions below.

template <typename T>
bool BlockingQueue<T>::push(T item) {
    (void)item;
    return false;
}

template <typename T>
bool BlockingQueue<T>::try_push(T item) {
    (void)item;
    return false;
}

template <typename T>
bool BlockingQueue<T>::pop(T& out) {
    (void)out;
    return false;
}

template <typename T>
bool BlockingQueue<T>::try_pop(T& out) {
    (void)out;
    return false;
}

template <typename T>
void BlockingQueue<T>::close() {}

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
