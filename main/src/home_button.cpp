#include "home_button.h"

namespace home_button {

  void enable(void) {
    auto thread = Thread("home_button_thread", threadCB);
    thread.start();
  }

  int threadCB(SceSize args, void* argp) {
    int cbid = sceKernelCreateCallback(
      "Exit Callback", 
      exitGame, 
      nullptr
    );
    sceKernelRegisterExitCallback(cbid);
    sceKernelSleepThreadCB();
  
    return 0;
  }

  int exitGame(int _, int __, void* ___) {
    sceKernelExitGame();
    return 0;
  }
}
