#!/bin/sh
workspaceRoot="$1"

path=$(echo "$workspaceRoot/target/mipsel-sony-psp/debug" | cut -d '/' -f4-)

pspsh debug.pspsh "$path"
      