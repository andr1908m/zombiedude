# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.27

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/andres/projects/learning-rust/zombiedude/main

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/andres/projects/learning-rust/zombiedude/target/main

# Utility rule file for copy_assets.

# Include any custom commands dependencies for this target.
include CMakeFiles/copy_assets.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/copy_assets.dir/progress.make

CMakeFiles/copy_assets:
	/usr/bin/cmake -E copy_directory /home/andres/projects/learning-rust/zombiedude/main/assets /home/andres/projects/learning-rust/zombiedude/target/main/assets

copy_assets: CMakeFiles/copy_assets
copy_assets: CMakeFiles/copy_assets.dir/build.make
.PHONY : copy_assets

# Rule to build all files generated by this target.
CMakeFiles/copy_assets.dir/build: copy_assets
.PHONY : CMakeFiles/copy_assets.dir/build

CMakeFiles/copy_assets.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/copy_assets.dir/cmake_clean.cmake
.PHONY : CMakeFiles/copy_assets.dir/clean

CMakeFiles/copy_assets.dir/depend:
	cd /home/andres/projects/learning-rust/zombiedude/target/main && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/andres/projects/learning-rust/zombiedude/main /home/andres/projects/learning-rust/zombiedude/main /home/andres/projects/learning-rust/zombiedude/target/main /home/andres/projects/learning-rust/zombiedude/target/main /home/andres/projects/learning-rust/zombiedude/target/main/CMakeFiles/copy_assets.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/copy_assets.dir/depend

