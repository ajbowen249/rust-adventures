#include <iostream>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

// Example 1:------------------------------------------------------------------
// Memory Peril
class Dog {
public:
    std::string name;

    void printName() {
        std::cout << "I am " << name << std::endl;
    }

    ~Dog() {
        std::cout << "End of " << name << std::endl;
    }
};

Dog* makeBadDog() {
    Dog spike;
    spike.name = "Spike";

    Dog* d = &spike;
    {
        Dog snoopy;
        snoopy.name = "Snoopy";
        d = &snoopy;
    }

    return d;
}

void badStack() {
    auto d = makeBadDog();
    d->printName();
}
// ----------------------------------------------------------------------------

// Example 2:------------------------------------------------------------------
// Verbosity of Mutability 1

void doWithStr(std::string&& str) {
    std::vector<std::string> strs;
    strs.push_back(std::move(str));
    std::cout << strs[0] << std::endl;
}

void passVec() {
    std::string str = "Test";
    doWithStr(std::move(str));

    // This prints out an empty string, which is potentially unexpcted, and
    // therefore risky. The contents of str have been moved elsewhere, but
    // the compiler isn't smart enough to know that.
    std::cout << "str is now: " << str << std::endl;
}

// ----------------------------------------------------------------------------

// Example 3:------------------------------------------------------------------
// Verbosity of Mutability 2

void justPrintISwear(std::string& str) {
    std::cout << str << std::endl;

    str = "I lied! ";
}

void beDeceived() {
    std::string str = "Innocent String";
    justPrintISwear(str);
    std::cout << str << std::endl;
}

// Note how, in Rust, fewer tokens are required to borrow a const reference.
// In C++, forgetting access modifiers leaves things unsafe. Forgetting them
// in Rust just means you need to add mutability when you really need it.

void justPrintISwearForReal(const std::string& str) {
    std::cout << str << std::endl;

    // This would finally cause a compiler error
    // str = "I lied!";
}

void beRelieved() {
    std::string str = "Actually Innocent String";
    justPrintISwearForReal(str);
    std::cout << str << std::endl;
}

// ----------------------------------------------------------------------------

// Example 4:------------------------------------------------------------------
// Mad Threads

// This is obviously a pretty bad idea
void madThreads() {
    for (int round = 0; round < 10; round++) {
        std::string intStr = "10000";
        std::vector<std::thread> threads;

        // Let's try to get to 20000:
        for (int i = 0; i < 100; i++) {
            threads.push_back(std::thread([&intStr, &threads, i]() {
                for (int j = 0; j < 100; j++) {
                    int value = atoi(intStr.c_str());
                    value++;
                    intStr = std::to_string(value);
                }
            }));
        }

        for (int i = 99; i >=0; i--) {
            threads[i].join();
        }

        std::cout << "(" << round << "/9) intStr: " << intStr << std::endl;
    }
}

void safeThreads() {
    for (int round = 0; round < 10; round++) {
        std::mutex mut;
        std::string intStr = "10000";
        std::vector<std::thread> threads;

        // Let's try to get to 20000:
        for (int i = 0; i < 100; i++) {
            threads.push_back(std::thread([&intStr, &threads, &mut, i]() {
                for (int j = 0; j < 100; j++) {
                    std::lock_guard<std::mutex> lock(mut);
                    int value = atoi(intStr.c_str());
                    value++;
                    intStr = std::to_string(value);
                }
            }));
        }

        for (int i = 99; i >=0; i--) {
            threads[i].join();
        }

        std::cout << "(" << round << "/9) safer intStr: " << intStr << std::endl;
    }
}

// ----------------------------------------------------------------------------

// Example 5:------------------------------------------------------------------
// Don't Ignore Errors (Unless you want to)

// This might fail. The bool indicates success
bool somethingThatMayFail() {
    return false;
}

bool somethingThatMayFailWithResult(std::string& outValue) {
    return false;
}

// This function produces zero compiler warnings.
void doWithFailures() {
    // Ignoring the result here. The compiler won't care.
    somethingThatMayFail();

    std::string value;
    somethingThatMayFailWithResult(value);

    // Print out our value, ignoring potential failure
    std::cout << value << std::endl;
}

// This handles errors, but you need to know to handle them
void handleFailures() {
    // Ignoring the result here. The compiler won't care.
    if (!somethingThatMayFail()) {
        std::cout << "Something failed. Aborting..." << std::endl;
        return;
    }

    std::string value;
    if (somethingThatMayFailWithResult(value)) {
        // Print out our value, ignoring potential failure
        std::cout << value << std::endl;
    }
}

// ----------------------------------------------------------------------------

int main() {
    std::cout << "Example 1: Memory Peril" << std::endl;
    badStack();
    std::cout << std::endl;

    std::cout << "Example 2: Verbosity of Mutability 1" << std::endl;
    passVec();
    std::cout << std::endl;

    std::cout << "Example 3: Verbosity of Mutability 2" << std::endl;
    beDeceived();
    beRelieved();
    std::cout << std::endl;

    std::cout << "Example 4: Mad Threads" << std::endl;
    madThreads();
    safeThreads();
    std::cout << std::endl;

    std::cout << "Example 5: Don't Ignore Errors (Unless you want to)" << std::endl;
    doWithFailures();
    handleFailures();
    std::cout << std::endl;

    return 0;
}
