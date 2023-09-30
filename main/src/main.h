#ifndef MAIN_H
#define MAIN_H

#include <string_view>
#include <new>
#include <string.h>
#include <malloc.h>

#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>

#include <pspmoduleinfo.h>
#include <pspdisplay.h>
#include <pspgu.h>
#include <pspgum.h>

#include <vector>
#include <memory>

#include <debug.h>
#include "home_button.h"
#include "types.h"

struct Vertex {
  float u;
  float v;
  u32 color;
  float x;
  float y;
  float z;
};

struct Image {
  i32 width;
  i32 height;
  i32 numberOfChannels;
  u8* pixels;

  Image(std::string_view path);
  virtual ~Image();
};

struct Texture {
  public:
    u32 width;
    u32 height;
    u32 powerWidth;
    u32 powerHeight;
    u32 *pixels;

    Texture() {};
    Texture(std::string_view path);
    void bind();
  private:
    u32* createSwizzledPixels(const Image& image);
    std::unique_ptr<u32[]> adjustToPowerWidth(const u32 *src);
    u32 * swizzle(const u8 *in, u32 width, u32 height);
};

struct Layout {
  const u32 width;
  const u32 height;
  const u32 pixelFormat;

  u32 memorySize();
};

int main(int argc, char *argv[]);
void resetTransform(ScePspFVector3&& v);


u32 pow2(u32 value);

#endif // MAIN_H
