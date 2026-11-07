// Build your own std::async from C++11 primitives: std::promise + std::thread.
//
//   template <class F, class... Args>
//   std::future<R> my_async(F&& f, Args&&... args);
//
// The promise is the write end of a one-shot channel and the future is the
// read end. my_async moves the promise into a new thread, runs f(args...)
// there, and stores either the result or the exception in the promise.
// The caller gets the future straight away and blocks only in get().
//
// Differences from std::async(std::launch::async, ...):
//   - The thread is detached, so ~future() does NOT block. std::async's
//     returned future blocks in its destructor until the task finishes.
//     Always call get() or wait() before main() returns, or a detached
//     thread may still be running while the program shuts down.
//   - No launch policy and no deferred execution: every call starts a
//     thread.
//
// C++11 notes:
//   - Lambdas cannot move-capture (init-capture is C++14), so the
//     move-only promise is passed to the thread as an argument instead.
//   - std::result_of computes the return type. It was deprecated in C++17
//     and removed in C++20 in favour of std::invoke_result.

#include <chrono>
#include <exception>
#include <future>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <thread>
#include <type_traits>
#include <utility>

namespace {

// Calls f(args...) and stores the outcome in p. Separate specialisation for
// void, because promise<void>::set_value() takes no argument.
template <typename R>
struct Fulfil {
    template <typename F, typename... Args>
    static void run(std::promise<R>& p, F& f, Args&... args) {
        p.set_value(f(std::move(args)...));
    }
};

template <>
struct Fulfil<void> {
    template <typename F, typename... Args>
    static void run(std::promise<void>& p, F& f, Args&... args) {
        f(std::move(args)...);
        p.set_value();
    }
};

// Thread entry point. Everything arrives by value: std::thread has already
// decay-copied (or moved) the promise, the callable and the arguments into
// the new thread, so nothing here refers to the caller's stack.
template <typename R, typename F, typename... Args>
void run_task(std::promise<R> p, F f, Args... args) {
    try {
        Fulfil<R>::run(p, f, args...);
    } catch (...) {
        // Carry the exception across threads; future::get() rethrows it.
        p.set_exception(std::current_exception());
    }
}

template <typename F, typename... Args>
std::future<typename std::result_of<typename std::decay<F>::type(
    typename std::decay<Args>::type...)>::type>
my_async(F&& f, Args&&... args) {
    typedef typename std::decay<F>::type Fn;
    typedef typename std::result_of<Fn(typename std::decay<Args>::type...)>::type R;

    std::promise<R> p;
    std::future<R> result = p.get_future();  // take the read end first

    std::thread t(&run_task<R, Fn, typename std::decay<Args>::type...>,
                  std::move(p), std::forward<F>(f), std::forward<Args>(args)...);
    t.detach();  // the shared state, not the thread, keeps the result alive
    return result;
}

// ---- demo tasks ------------------------------------------------------------

int square(int x) { return x * x; }

int fails(int x) {
    if (x < 0) throw std::invalid_argument("negative input: " + std::to_string(x));
    return x;
}

std::string slow_echo(std::string s, int ms) {
    std::this_thread::sleep_for(std::chrono::milliseconds(ms));
    return s;
}

}  // namespace

int main() {
    // 1. A value comes back through the future.
    std::future<int> f1 = my_async(square, 7);
    std::cout << "square(7) = " << f1.get() << '\n';

    // 2. Lambdas and void tasks: get() just waits.
    int counter = 0;
    std::future<void> f2 = my_async([&counter] { counter = 42; });
    f2.get();  // also makes the write to counter visible to this thread
    std::cout << "void task set counter to " << counter << '\n';

    // 3. Exceptions thrown on the worker are rethrown by get().
    std::future<int> f3 = my_async(fails, -1);
    try {
        (void)f3.get();  // only called to rethrow; there is no value
    } catch (const std::invalid_argument& e) {
        std::cout << "caught from worker: " << e.what() << '\n';
    }

    // 4. Move-only arguments are moved all the way into the task.
    std::unique_ptr<int> owned(new int(5));  // no std::make_unique in C++11
    std::future<int> f4 = my_async(
        [](std::unique_ptr<int> p) { return *p * 10; }, std::move(owned));
    std::cout << "moved unique_ptr, task returned " << f4.get()
              << " (caller's pointer is now " << (owned ? "set" : "null") << ")\n";

    // 5. Tasks run concurrently: three 200 ms tasks take ~200 ms, not ~600.
    auto start = std::chrono::steady_clock::now();
    std::future<std::string> a = my_async(slow_echo, std::string("a"), 200);
    std::future<std::string> b = my_async(slow_echo, std::string("b"), 200);
    std::future<std::string> c = my_async(slow_echo, std::string("c"), 200);
    std::string joined = a.get() + b.get() + c.get();
    auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(
                  std::chrono::steady_clock::now() - start).count();
    std::cout << "three 200 ms tasks returned \"" << joined << "\" in " << ms
              << " ms\n";
}
