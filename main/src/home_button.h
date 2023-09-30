#ifndef HOME_BUTTON_H
#define HOME_BUTTON_H

#include <pspkernel.h>
#include <pspthreadman.h>
#include <sstream>
#include <string_view>

#include "thread.h"


namespace home_button {
  void enable();
  int threadCB(SceSize args, void* argp);
  int exitGame(int _, int __, void* ___);
}

#endif
