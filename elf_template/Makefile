TARGET = template
OBJS = main.o



INCDIR = 
CFLAGS = -O2 -Wall -target mips -mcpu=mips2 -msingle-float -mlittle-endian -mno-check-zero-division
CXXFLAGS = $(CFLAGS) -fno-exceptions -fno-rtti
ASFLAGS = $(CFLAGS)

LIBDIR = ../target/mipsel-sony-psp/release
LDFLAGS = 
LIBS = -lrust_and_c

EXTRA_TARGETS = EBOOT.PBP
PSP_EBOOT_TITLE = Template

PSPSDK=$(shell psp-config --pspsdk-path)
include $(PSPSDK)/lib/build.mak

CC = clang