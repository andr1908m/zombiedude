#!/bin/sh
workspaceRoot="$1"
prx_file="$2"

path="target/mipsel-sony-psp/debug"

pspsh debug.pspsh "$path" "$prx_file"
      