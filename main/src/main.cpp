#include "main.h"

PSP_MODULE_INFO("Texture Sample", PspModuleInfoAttr::PSP_MODULE_USER, 1, 1);
//PSP_MAIN_THREAD_ATTR(THREAD_ATTR_USER | THREAD_ATTR_VFPU);

#define gpu_aligned __attribute__((aligned(16)))

const i32 KiB = 1024;

class DisplayContext {
  public:
    void clear() {
      sceGuStart(GU_DIRECT, displayList);
    }
  private:
    u32 gpu_aligned displayList[256 * KiB];
};

u32 Layout::memorySize() {
  switch (pixelFormat) {
    case GU_PSM_T4:
      return (width * height) >> 1;

    case GU_PSM_T8:
      return width * height;

    case GU_PSM_5650:
    case GU_PSM_5551:
    case GU_PSM_4444:
    case GU_PSM_T16:
      return 2 * width * height;

    case GU_PSM_8888:
    case GU_PSM_T32:
      return 4 * width * height;

    default:
      return 0;
  }
}

class VRAM {
  public:
    u32* allocate(Layout layout) {
      auto offset = reinterpret_cast<uintptr_t>(allocateRelative(layout));
      auto edramAddr = reinterpret_cast<uintptr_t>(sceGeEdramGetAddr());
      return reinterpret_cast<u32*>(edramAddr + offset);
    }

    void* allocateRelative(Layout layout) {
      u32 memSize = layout.memorySize();

      auto result = reinterpret_cast<void*>(staticOffset);
      staticOffset += memSize;

      return result;
    }

    void deallocate() {
      staticOffset = 0;
    }

  private:
    u32 staticOffset;
};

VRAM vram = VRAM { };
DisplayContext displayContext = DisplayContext { };

class Graphics {
  public:
    Graphics() {
      initialize();
      waitForGPU();
      waitForVerticalBlank();
      turnOnDisplay();
    }

    virtual ~Graphics() {
      terminate();
    }

  private:
    void initialize() {
      initializeDisplayContext();
      specifySettings();
      finishDisplayContext();
    }

    void initializeDisplayContext() {
      sceGuInit();
      displayContext.clear();
    }

    void specifySettings() {
      specifyRenderingBounds();
      enableOptions();
    }

    void specifyRenderingBounds() {
      specifyBuffers(PSP_SCR_HEIGHT, PSP_SCR_WIDTH);
      specifyViewport(PSP_SCR_HEIGHT, PSP_SCR_WIDTH);
      specifyDepthRange({ .near = (64 * KiB) - 1, .far = 0 });
      specifyScissorRegion({ .left = 0, .top = 0, PSP_SCR_WIDTH, PSP_SCR_HEIGHT });
    }


    void specifyBuffers(u32 height, u32 width) {
      specifyDrawBuffer(height);
      specifyDisplayBuffer(height, width);
      specifyDepthBuffer(height);
    }

    void specifyDrawBuffer(u32 height) {
      auto drawBuffer = vram.allocateRelative(
          Layout{ PSP_BUF_WIDTH, height, GU_PSM_8888 });
      sceGuDrawBuffer(GU_PSM_8888, drawBuffer, PSP_BUF_WIDTH);
    }

    void specifyDisplayBuffer(u32 height, u32 width) {
      auto displayBuffer = vram.allocateRelative(
          Layout { PSP_BUF_WIDTH, height, GU_PSM_8888 });
      sceGuDispBuffer(width, height, displayBuffer, PSP_BUF_WIDTH);
    }

    void specifyDepthBuffer(u32 height) {
      auto depthBuffer = vram.allocateRelative(
          Layout { PSP_BUF_WIDTH, height, GU_PSM_4444 });
      sceGuDepthBuffer(depthBuffer, PSP_BUF_WIDTH);
    }

    void specifyViewport(u32 viewportHeight, u32 viewportWidth) {
      sceGuOffset(2048 - (viewportWidth / 2), 2048 - (viewportHeight / 2));
      sceGuViewport(2048, 2048, viewportWidth, viewportHeight);
    }

    struct DepthRange {
        i32 near;
        i32 far;
    };

    void specifyDepthRange(DepthRange range) {
      sceGuDepthRange(range.near, range.far);
    }

    struct ScissorRegion {
        i32 left;
        i32 top;
        u32 screenWidth;
        u32 screenHeight;
    };

    void specifyScissorRegion(ScissorRegion region) {
      sceGuScissor(region.left, region.top, region.screenWidth,
          region.screenHeight);
      sceGuEnable(GU_SCISSOR_TEST);
    }

    void enableOptions() {
      enableDepthTest();
      enableSmoothShading();
      enableCullFace();
      enableTextures();
      enableClipPlanes();
      enableBlending();
    }

    void enableDepthTest() {
      sceGuEnable(GU_DEPTH_TEST);
      sceGuDepthFunc(GU_LESS);
      sceGuDepthMask(false);
    }

    void enableCullFace() {
      sceGuEnable(GU_CULL_FACE);
      sceGuFrontFace(GU_CW);
    }

    void enableSmoothShading() {
      sceGuShadeModel(GU_SMOOTH);
    }

    void enableTextures() {
      sceGuEnable(GU_TEXTURE_2D);
    }

    void enableClipPlanes() {
      sceGuEnable(GU_CLIP_PLANES);
    }

    void enableBlending() {
      sceGuEnable(GU_BLEND);
      u32 srcfix = 1;
      u32 destfix = 0;
      sceGuBlendFunc(GU_ADD, GU_SRC_ALPHA, GU_ONE_MINUS_SRC_ALPHA, srcfix, destfix);
    }

    void finishDisplayContext() {
      sceGuFinish();
    }

    void waitForGPU() {
      sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);
    }

    void waitForVerticalBlank() {
      sceDisplayWaitVblankStart();
    }

    enum DisplayStatus {
      ON = true, OFF = false
    };

    void turnOnDisplay() {
      sceGuDisplay(DisplayStatus::ON);
    }

    void terminate() {
      sceGuTerm();
    }
};

template<size_t V, size_t I>
struct Mesh {
    Vertex gpu_aligned vertices[V];
    u16 gpu_aligned indices[I];
};

template<size_t V, size_t I>
class Painter {
  public:
    Painter(Mesh<V,I>& mesh): mesh{mesh} {}

    void paint(Texture t, ScePspFVector3 position) {
      moveToWorldSpace(position);
      t.bind();
      i32 vtype = GU_INDEX_16BIT |
          GU_TEXTURE_32BITF |
          GU_COLOR_8888 |
          GU_VERTEX_32BITF |
          GU_TRANSFORM_3D;
      sceGumDrawArray(GU_TRIANGLES, vtype, I, mesh.indices, mesh.vertices);
    }

    void moveToWorldSpace(ScePspFVector3 v) {
      sceGumMatrixMode(GU_MODEL);
      sceGumLoadIdentity();
      sceGumTranslate(&v);
    }

  private:
    Mesh<V,I> mesh;
};



class GameLoop {
  public:
    void run() {
      initialize();
      while (running) {
        start();
        onFrame();
        end();
      }
    }
    virtual ~GameLoop() {};

  protected:
    virtual void initialize() = 0;
    virtual void onFrame() = 0;

  private:
    void start() {
      displayContext.clear();
    }

    void end() {
      sceGuFinish();
      sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);
      sceDisplayWaitVblankStart();
      sceGuSwapBuffers();
    }

    bool running = true;
};


class Main : public GameLoop {
  public:
    virtual void initialize() {
      home_button::enable();

      sceGumMatrixMode(GU_PROJECTION);
      sceGumLoadIdentity();
      sceGumPerspective(
          toDegrees(1.0f/8.0f),
          16.0f/9.0f,
          0.1f, 500.0f
      );
      sceKernelDcacheWritebackAll();
    }

    f32 toDegrees(f32 t) {
      return t * 360.0f;
    }

    virtual void onFrame() {
      int black = 0xFF000000;
      sceGuClearColor(black);
      sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT);

      lookFrom({
        .position = { 10.0f*static_cast<f32>(sin(i)), 0.0f, 10.0f*static_cast<f32>(cos(i)) },
        .target = { 0, 0, 0 }
      });
      Painter cubePainter(cube);
      cubePainter.paint(textures["container"], { 1, 0, 1 });
//      cubePainter.paint(textures["circle"], { -1, 0, 0 });

      i += 0.01;
    }

    struct Camera {
        ScePspFVector3 position;
        ScePspFVector3 target;
    };

    void lookFrom(Camera something) {
      sceGumMatrixMode(GU_VIEW);
      sceGumLoadIdentity();
      ScePspMatrix3 lookAt = {
          .fm = {
              something.position,
              something.target,
              { 0, 1.0f, 0 }
          }
      };
      sceGumLookAt(&(lookAt.fm.x), &(lookAt.fm.y), &(lookAt.fm.z));

    }

  private:
    f32 i = 0.01;
    Graphics g;
    std::unordered_map<std::string, Texture> textures = {
        { "container" , Texture("assets/img/container.jpg") },
        { "circle" , Texture("assets/img/circle.png") }
    };
    Mesh<36,36> cube = {
        .vertices = {
            // Front face
            {0, 0, 0xFF0000FF, -1, -1, -1},
            {1, 0, 0xFF0000FF, 1, -1, -1},
            {1, 1, 0xFF0000FF, 1, 1, -1},
            {1, 1, 0xFF0000FF, 1, 1, -1},
            {0, 1, 0xFF0000FF, -1, 1, -1},
            {0, 0, 0xFF0000FF, -1, -1, -1},

            // Back face
            {0, 0, 0xFF00FF00, -1, -1, 1},
            {1, 0, 0xFF00FF00, 1, -1, 1},
            {1, 1, 0xFF00FF00, 1, 1, 1},
            {1, 1, 0xFF00FF00, 1, 1, 1},
            {0, 1, 0xFF00FF00, -1, 1, 1},
            {0, 0, 0xFF00FF00, -1, -1, 1},

            // Left face
            {0, 0, 0xFFFF0000, -1, 1, 1},
            {1, 0, 0xFFFF0000, -1, 1, -1},
            {1, 1, 0xFFFF0000, -1, -1, -1},
            {1, 1, 0xFFFF0000, -1, -1, -1},
            {0, 1, 0xFFFF0000, -1, -1, 1},
            {0, 0, 0xFFFF0000, -1, 1, 1},

            // Right face
            {0, 0, 0xFFFFFF00, 1, 1, 1},
            {1, 0, 0xFFFFFF00, 1, 1, -1},
            {1, 1, 0xFFFFFF00, 1, -1, -1},
            {1, 1, 0xFFFFFF00, 1, -1, -1},
            {0, 1, 0xFFFFFF00, 1, -1, 1},
            {0, 0, 0xFFFFFF00, 1, 1, 1},

            // Top face
            {0, 0, 0xFF00FFFF, -1, 1, -1},
            {1, 0, 0xFF00FFFF, 1, 1, -1},
            {1, 1, 0xFF00FFFF, 1, 1, 1},
            {1, 1, 0xFF00FFFF, 1, 1, 1},
            {0, 1, 0xFF00FFFF, -1, 1, 1},
            {0, 0, 0xFF00FFFF, -1, 1, -1},

            // Bottom face
            {0, 0, 0xFF0000FF, -1, -1, -1},
            {1, 0, 0xFF0000FF, 1, -1, -1},
            {1, 1, 0xFF0000FF, 1, -1, 1},
            {1, 1, 0xFF0000FF, 1, -1, 1},
            {0, 1, 0xFF0000FF, -1, -1, 1},
            {0, 0, 0xFF0000FF, -1, -1, -1},
        },
        .indices = {
            0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35
        },
    };
};


int main(int argc, char *argv[]) {
  auto main = std::make_unique<Main>();
  main->run();
  return 0;
}

Texture::Texture(std::string_view path) {
  Image image(path);
  width = image.width;
  height = image.height;
  powerWidth = pow2(image.width);
  powerHeight = pow2(image.height);
  pixels = createSwizzledPixels(image);
  sceKernelDcacheWritebackInvalidateAll();
}

u32* Texture::createSwizzledPixels(const Image &image) {
  auto pixels = reinterpret_cast<const u32*>(image.pixels);
  auto adj = reinterpret_cast<const u8*>(adjustToPowerWidth(pixels).get());
  u32 *swizzled = swizzleInGPU(adj, powerWidth * 4, powerHeight);
  return swizzled;
}

std::unique_ptr<u32[]> Texture::adjustToPowerWidth(const u32 *src) {
  std::unique_ptr<u32[]> result {
    new (std::align_val_t { 16 }) u32[powerHeight * powerWidth * 4]
  };
  for (size_t y = 0; y < height; y++) {
    for (size_t x = 0; x < width; x++) {
      result[x + y * powerWidth] = src[x + y * width];
    }
  }
  return result;
}

u32* Texture::swizzleInGPU(const u8 *in, u32 width, u32 height) {
  u32 *out = vram.allocate(Layout { powerWidth, powerHeight, GU_PSM_8888 });
  u32 width_blocks = (width / 16);
  u32 height_blocks = (height / 8);

  u32 src_pitch = (width - 16) / 4;
  u32 src_row = width * 8;

  const u8 *ysrc = in;
  u32 *dst = out;

  for (size_t blocky = 0; blocky < height_blocks; blocky++) {
    const u8 *xsrc = ysrc;
    for (size_t blockx = 0; blockx < width_blocks; blockx++) {
      const u32 *src = reinterpret_cast<const u32*>(xsrc);
      for (size_t j = 0; j < 8; ++j) {
        for (size_t k = 0; k < 4; ++k) {
          *dst = *src;
          dst += 1;
          src += 1;
        }
        src += src_pitch;
      }
      xsrc += 16;
    }
    ysrc += src_row;
  }
  return out;
}

u32 pow2(u32 value) {
  u32 poweroftwo = 1;
  while (poweroftwo < value) {
    poweroftwo <<= 1;
  }
  return poweroftwo;
}

void Texture::bind() {
  bool enableSwizzle = true;
  i32 maxmips = 0;
  sceGuTexMode(GU_PSM_8888, maxmips, 0, enableSwizzle);
  sceGuTexFunc(GU_TFX_MODULATE, GU_TCC_RGBA);
  sceGuTexFilter(GU_LINEAR, GU_LINEAR);
  sceGuTexWrap(GU_REPEAT, GU_REPEAT);
  i32 mipmapLevel = 0;
  sceGuTexImage(mipmapLevel, powerWidth, powerHeight, powerWidth, pixels);
  sceGuAmbientColor(0xffffffff);
}

Image::Image(std::string_view path) {
  stbi_set_flip_vertically_on_load(true);
  pixels = stbi_load(path.data(),
      &width, &height, &numberOfChannels, STBI_rgb_alpha);
  ensure(pixels != nullptr, "Failed to load image, was null");
}

Image::~Image() {
  stbi_image_free(pixels);
}


