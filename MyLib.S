	.set noreorder

#include "pspimport.s"

// Build files
// MyLib_0000.o MyLib_0001.o MyLib_0002.o 

#ifdef F_MyLib_0000
	IMPORT_START "MyLib",0x00090000
#endif
#ifdef F_MyLib_0001
	IMPORT_FUNC  "MyLib",0x7FDB69D8,whatever
#endif
#ifdef F_MyLib_0002
	IMPORT_FUNC  "MyLib",0x915F2BC2,hi
#endif
