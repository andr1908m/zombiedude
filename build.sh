#!/bin/sh
build_std_flags="build-std=core,compiler_builtins,alloc,panic_unwind,panic_abort"

cargo build -Z "$build_std_flags" \
  --target mipsel-sony-psp

output_folder="./target/mipsel-sony-psp/debug"

get_elfs() {
  find "$output_folder" -maxdepth 1 -type f ! -name "*\.*" 
}

for f in $(get_elfs); do
  fname="$(basename $f)"
  echo "!!! GENERATING PRX FOR $fname !!!"
  prxgen "$f" "$output_folder/$fname.elf"
  echo "!!! DONE GENERATING PRX FOR $fname !!!"
done