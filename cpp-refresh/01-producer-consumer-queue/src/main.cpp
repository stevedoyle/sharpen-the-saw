// pcq-demo: read lines from stdin, pass them through a BlockingQueue as
// move-only jobs, and have N consumer threads write results to stdout.
//
//   seq 1 1000 | ./build/debug/pcq-demo 4 | sort -n | tail
//
// Diagnostics go to stderr so stdout stays pipeable.

#include <pcq/blocking_queue.hpp>

#include <cstdlib>
#include <iostream>
#include <memory>
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

}  // namespace

int main(int argc, char* argv[]) {
    unsigned consumers = 2;
    if (argc > 1) {
        consumers = static_cast<unsigned>(std::strtoul(argv[1], nullptr, 10));
        if (consumers == 0) {
            std::cerr << "usage: pcq-demo [consumers>0]\n";
            return EXIT_FAILURE;
        }
    }

    pcq::BlockingQueue<JobPtr> queue(64);

    // TODO(capstone):
    //   1. Start `consumers` threads that pop JobPtr until pop() returns
    //      false, and write each result to stdout (guard std::cout).
    //   2. On this thread, read stdin line by line and push
    //      JobPtr(new Job{...}) -- no std::make_unique in C++11.
    //   3. close() the queue, then join every consumer.
    (void)queue;
    std::cerr << "pcq-demo: not implemented yet (" << consumers
              << " consumers requested)\n";
    return EXIT_SUCCESS;
}
