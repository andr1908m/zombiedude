#include <pspdebug.h>
#include <pspthreadman.h>
#include <sstream>
#include <string_view>

#define assertPSP(condition, ...) {\
    if(!(condition)){\
      pspDebugScreenInit();\
      pspDebugScreenClear();\
      pspDebugScreenPrintf("Assertion failed: " __VA_ARGS__);\
      pspDebugScreenPrintf(" at line %d in file %s", __LINE__, __FILE__);\
      sceKernelSleepThread();\
    }\
}

#define require assertPSP
#define invariant assertPSP
#define ensure assertPSP


