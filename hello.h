#include <pspmoduleexport.h>
#define NULL ((void *) 0)

extern int module_start;
extern int module_info;
static const unsigned int __syslib_exports[4] __attribute__((section(".rodata.sceResident"))) = {
	0xD632ACDB,
	0xF01D73A7,
	(unsigned int) &module_start,
	(unsigned int) &module_info,
};

extern int whatever;
extern int hi;
static const unsigned int __MyLib_exports[4] __attribute__((section(".rodata.sceResident"))) = {
	0x7FDB69D8,
	0x915F2BC2,
	(unsigned int) &whatever,
	(unsigned int) &hi,
};

const struct _PspLibraryEntry __library_exports[2] __attribute__((section(".lib.ent"), used)) = {
	{ NULL, 0x0000, 0x8000, 4, 1, 1, (unsigned int *) &__syslib_exports },
	{ "MyLib", 0x0000, 0x0001, 4, 0, 2, (unsigned int *) &__MyLib_exports },
};
