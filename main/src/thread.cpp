#include "thread.h"

Thread::Thread(std::string_view name, SceKernelThreadEntry entry)
    : name(name), entry { entry } {
  const int KB = 1000;
  int priority = 17;
  int stackSize = 4 * KB;
  thid = createThread(priority, stackSize);
}

int Thread::createThread(int priority, int stackSize) {
  int thid = sceKernelCreateThread(name.data(), entry, priority, stackSize,
      PspThreadAttributes::PSP_THREAD_ATTR_USER, nullptr);
  ensure(
      thid >= 0, "Failed to create %s with error: 0x%08X", name.data(), thid
  );
  return thid;
}

void Thread::startWith(SceSize arglen, void *argp) {
  sceKernelStartThread(thid, arglen, argp);
}

void Thread::start() {
  sceKernelStartThread(thid, 0, nullptr);
}


