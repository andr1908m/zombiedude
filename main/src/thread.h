#ifndef THREAD_H_
#define THREAD_H_

#include <debug.h>
#include <pspthreadman.h>
#include <sstream>
#include <string_view>
#include <vector>

class Thread {
  public:
    Thread(std::string_view name, SceKernelThreadEntry entry);

    void startWith(SceSize arglen, void *argp);

    void start();

  private:
    std::string_view name;
    SceKernelThreadEntry entry;
    int thid;

    int createThread(int priority, int stackSize);
};

#endif
