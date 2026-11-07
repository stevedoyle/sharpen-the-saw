// pcq-demo: read lines from stdin, pass them through a BlockingQueue as
// move-only jobs, and have N consumer threads write results to stdout.
//
// Each output line is "<seq>\t<line>", where <seq> is the 1-based input
// line number. Consumers run concurrently, so output order varies between
// runs; restore input order with `sort -n | cut -f2-`:
//
//   seq 1 1000 | ./build/debug/pcq-demo 4 | sort -n | tail
//
// Diagnostics go to stderr so stdout stays pipeable.

#include <blocking_queue.hpp>

#include <cerrno>
#include <cstdlib>
#include <exception>
#include <iostream>
#include <memory>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

namespace {

// Move-only unit of work. Held by std::unique_ptr so ownership transfer
// through the queue is explicit.
struct Job {
    std::size_t seq;
    std::string line;
};

using JobPtr = std::unique_ptr<Job>;

// Bounded so a fast reader cannot buffer all of stdin ahead of slow
// consumers.
const std::size_t kQueueCapacity = 64;

// Parses a strictly positive decimal count. Returns false on anything else
// ("", "0", "-1", "4x", out of range).
bool parse_consumers(const char* s, unsigned& out) {
    if (*s == '\0' || *s == '-') return false;
    char* end = nullptr;
    errno = 0;
    unsigned long n = std::strtoul(s, &end, 10);
    if (errno != 0 || *end != '\0' || n == 0 || n > 1024) return false;
    out = static_cast<unsigned>(n);
    return true;
}

// Pops jobs until the queue is closed and drained, writing each to `out`.
// `out_mutex` keeps each line whole when several consumers share `out`.
void consume(pcq::BlockingQueue<JobPtr>& queue, std::ostream& out,
             std::mutex& out_mutex) {
    JobPtr job;
    while (queue.pop(job)) {
        std::lock_guard<std::mutex> lock(out_mutex);
        out << job->seq << '\t' << job->line << '\n';
    }
}

// On scope exit, closes the queue and joins every started worker. This runs
// on the normal path and when an exception unwinds, so no joinable
// std::thread is ever destroyed (which would call std::terminate). Closing
// first matters: consumers blocked in pop() only return once it is closed.
class WorkerGuard {
public:
    WorkerGuard(pcq::BlockingQueue<JobPtr>& queue,
                std::vector<std::thread>& workers)
        : queue_(queue), workers_(workers) {}
    WorkerGuard(const WorkerGuard&) = delete;
    WorkerGuard& operator=(const WorkerGuard&) = delete;

    ~WorkerGuard() {
        queue_.close();
        for (auto& t : workers_) {
            if (t.joinable()) t.join();
        }
    }

private:
    pcq::BlockingQueue<JobPtr>& queue_;
    std::vector<std::thread>& workers_;
};

// Runs the pipeline: `consumers` threads pop and print while this thread
// reads stdin and pushes. Returns once every job has been written.
void run(unsigned consumers) {
    pcq::BlockingQueue<JobPtr> queue(kQueueCapacity);
    std::mutex out_mutex;
    std::vector<std::thread> workers;
    WorkerGuard guard(queue, workers);  // declared last, destroyed first

    workers.reserve(consumers);
    for (unsigned i = 0; i < consumers; ++i) {
        workers.emplace_back(consume, std::ref(queue), std::ref(std::cout),
                             std::ref(out_mutex));
    }

    // Producer: this thread. make_unique<Job>(a, b) on an aggregate relies
    // on C++20 parenthesised aggregate initialisation.
    std::size_t seq = 0;
    std::string line;
    while (std::getline(std::cin, line)) {
        if (!queue.push(std::make_unique<Job>(++seq, std::move(line)))) break;
    }
}  // guard: close() and join all consumers

}  // namespace

int main(int argc, char* argv[]) {
    unsigned consumers = 2;
    if (argc > 2 || (argc == 2 && !parse_consumers(argv[1], consumers))) {
        std::cerr << "usage: pcq-demo [consumers, 1-1024]\n";
        return EXIT_FAILURE;
    }

    // Catching here guarantees stack unwinding, so WorkerGuard runs even
    // when an exception (e.g. std::system_error from std::thread, or
    // std::bad_alloc) escapes run().
    try {
        run(consumers);
    } catch (const std::exception& e) {
        std::cerr << "pcq-demo: " << e.what() << '\n';
        return EXIT_FAILURE;
    }

    std::cout.flush();
    if (!std::cout) {
        std::cerr << "pcq-demo: error writing to stdout\n";
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}
