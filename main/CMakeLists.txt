cmake_minimum_required(VERSION 3.5)

project(main)

# -- global flags and toolchain --
include(./build/conan_toolchain.cmake)
include($ENV{PSPDEV}/psp/share/pspdev.cmake)

set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_BUILD_TYPE Debug)

# -- module --
file(GLOB_RECURSE SRC_FILES src/*.cpp)

add_executable(${PROJECT_NAME} ${SRC_FILES})

add_dependencies(${PROJECT_NAME} copy_assets)

add_custom_target(copy_assets
  COMMAND 
    ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_LIST_DIR}/assets 
      ${CMAKE_CURRENT_BINARY_DIR}/assets
)

create_pbp_file(
  TARGET ${PROJECT_NAME}
  ICON_PATH NULL
  PREVIEW_PATH NULL
  TITLE Zombiedude
  BUILD_PRX
  ENC_PRX
)

# -- flags --
target_include_directories(${PROJECT_NAME} 
  PRIVATE ${CMAKE_INCLUDE_PATH} include/ src/)

# -- task automation --

add_custom_target(run_on_device
  COMMAND 
    pgrep -x usbhostfs_pc || usbhostfs_pc -d
  COMMAND
    pspsh -e "main.prx"
)

# -- dependencies --
find_package(stb REQUIRED)

target_link_libraries(${PROJECT_NAME} PRIVATE
  pspdebug 
  pspdisplay 
  pspge 
  pspgu 
  pspgum
  stb::stb
)


  