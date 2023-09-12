#![no_main]
#![no_std]

mod macros;


use core::{mem::size_of, fmt::Display, hash::Hasher, ptr::{null_mut, null}};
use psp::sys::{SceUid, SceKernelLMOption, sceKernelLoadModule, sceKernelLoadModuleMs, sceKernelStartModule};

psp::module!("Test load", 0, 1);


psp_extern! {
  #![name = "test_create"]
  #![flags = 0x0]
  #![version = (0x00, 0x01)]

  #[psp(0x0845f1cf)]
  pub fn library_call();

  #[psp(0x28fa2125)]
  pub fn library_call_2(b:i32);
}

fn psp_main() {
  load_module("ms0:/test_create.prx");
  unsafe {
    library_call();
    library_call_2(2);
  }
  // psp::dprintln!("Hello, world!");
  // let test_create = 
  // unsafe {
  //   library_call();
  // }
}

const KERNEL_ERROR_CODES: [(u32, &'static str); 198] = [
  (0, "OK"),   
  (0x80020001, "ERROR"),  
  (0x80020002, "NOTIMP"),   
  (0x80020032, "ILLEGAL_EXPCODE"),   
  (0x80020033, "EXPHANDLER_NOUSE"),   
  (0x80020034, "EXPHANDLER_USED"),   
  (0x80020035, "SYCALLTABLE_NOUSED"),   
  (0x80020036, "SYCALLTABLE_USED"),   
  (0x80020037, "ILLEGAL_SYSCALLTABLE"),   
  (0x80020038, "ILLEGAL_PRIMARY_SYSCALL_NUMBER"),   
  (0x80020039, "PRIMARY_SYSCALL_NUMBER_INUSE"),   
  (0x80020064, "ILLEGAL_CONTEXT"),   
  (0x80020065, "ILLEGAL_INTRCODE"),   
  (0x80020066, "CPUDI"),   
  (0x80020067, "FOUND_HANDLER"),   
  (0x80020068, "NOTFOUND_HANDLER"),   
  (0x80020069, "ILLEGAL_INTRLEVEL"),   
  (0x8002006a, "ILLEGAL_ADDRESS"),   
  (0x8002006b, "ILLEGAL_INTRPARAM"),   
  (0x8002006c, "ILLEGAL_STACK_ADDRESS"),   
  (0x8002006d, "ALREADY_STACK_SET"),   
  (0x80020096, "NO_TIMER"),   
  (0x80020097, "ILLEGAL_TIMERID"),   
  (0x80020098, "ILLEGAL_SOURCE"),   
  (0x80020099, "ILLEGAL_PRESCALE"),   
  (0x8002009a, "TIMER_BUSY"),   
  (0x8002009b, "TIMER_NOT_SETUP"),   
  (0x8002009c, "TIMER_NOT_INUSE"),   
  (0x800200a0, "UNIT_USED"),   
  (0x800200a1, "UNIT_NOUSE"),   
  (0x800200a2, "NO_ROMDIR"),   
  (0x800200c8, "IDTYPE_EXIST"),   
  (0x800200c9, "IDTYPE_NOT_EXIST"),   
  (0x800200ca, "IDTYPE_NOT_EMPTY"),   
  (0x800200cb, "UNKNOWN_UID"),   
  (0x800200cc, "UNMATCH_UID_TYPE"),   
  (0x800200cd, "ID_NOT_EXIST"),   
  (0x800200ce, "NOT_FOUND_UIDFUNC"),   
  (0x800200cf, "UID_ALREADY_HOLDER"),   
  (0x800200d0, "UID_NOT_HOLDER"),   
  (0x800200d1, "ILLEGAL_PERM"),   
  (0x800200d2, "ILLEGAL_ARGUMENT"),   
  (0x800200d3, "ILLEGAL_ADDR"),   
  (0x800200d4, "OUT_OF_RANGE"),   
  (0x800200d5, "MEM_RANGE_OVERLAP"),   
  (0x800200d6, "ILLEGAL_PARTITION"),   
  (0x800200d7, "PARTITION_INUSE"),   
  (0x800200d8, "ILLEGAL_MEMBLOCKTYPE"),   
  (0x800200d9, "MEMBLOCK_ALLOC_FAILED"),   
  (0x800200da, "MEMBLOCK_RESIZE_LOCKED"),   
  (0x800200db, "MEMBLOCK_RESIZE_FAILED"),   
  (0x800200dc, "HEAPBLOCK_ALLOC_FAILED"),   
  (0x800200dd, "HEAP_ALLOC_FAILED"),   
  (0x800200de, "ILLEGAL_CHUNK_ID"),   
  (0x800200df, "NOCHUNK"),   
  (0x800200e0, "NO_FREECHUNK"),   
  (0x8002012c, "LINKERR"),   
  (0x8002012d, "ILLEGAL_OBJECT"),   
  (0x8002012e, "UNKNOWN_MODULE"),   
  (0x8002012f, "NOFILE"),   
  (0x80020130, "FILEERR"),   
  (0x80020131, "MEMINUSE"),   
  (0x80020132, "PARTITION_MISMATCH"),   
  (0x80020133, "ALREADY_STARTED"),   
  (0x80020134, "NOT_STARTED"),   
  (0x80020135, "ALREADY_STOPPED"),   
  (0x80020136, "CAN_NOT_STOP"),   
  (0x80020137, "NOT_STOPPED"),   
  (0x80020138, "NOT_REMOVABLE"),   
  (0x80020139, "EXCLUSIVE_LOAD"),   
  (0x8002013a, "LIBRARY_NOT_YET_LINKED"),   
  (0x8002013b, "LIBRARY_FOUND"),   
  (0x8002013c, "LIBRARY_NOTFOUND"),   
  (0x8002013d, "ILLEGAL_LIBRARY"),   
  (0x8002013e, "LIBRARY_INUSE"),   
  (0x8002013f, "ALREADY_STOPPING"),   
  (0x80020140, "ILLEGAL_OFFSET"),   
  (0x80020141, "ILLEGAL_POSITION"),   
  (0x80020142, "ILLEGAL_ACCESS"),   
  (0x80020143, "MODULE_MGR_BUSY"),   
  (0x80020144, "ILLEGAL_FLAG"),   
  (0x80020145, "CANNOT_GET_MODULELIST"),   
  (0x80020146, "PROHIBIT_LOADMODULE_DEVICE"),   
  (0x80020147, "PROHIBIT_LOADEXEC_DEVICE"),   
  (0x80020148, "UNSUPPORTED_PRX_TYPE"),   
  (0x80020149, "ILLEGAL_PERM_CALL"),   
  (0x8002014a, "CANNOT_GET_MODULE_INFORMATION"),   
  (0x8002014b, "ILLEGAL_LOADEXEC_BUFFER"),   
  (0x8002014c, "ILLEGAL_LOADEXEC_FILENAME"),   
  (0x8002014d, "NO_EXIT_CALLBACK"),   
  (0x80020190, "NO_MEMORY"),   
  (0x80020191, "ILLEGAL_ATTR"),   
  (0x80020192, "ILLEGAL_ENTRY"),   
  (0x80020193, "ILLEGAL_PRIORITY"),   
  (0x80020194, "ILLEGAL_STACK_SIZE"),   
  (0x80020195, "ILLEGAL_MODE"),   
  (0x80020196, "ILLEGAL_MASK"),   
  (0x80020197, "ILLEGAL_THID"),   
  (0x80020198, "UNKNOWN_THID"),   
  (0x80020199, "UNKNOWN_SEMID"),   
  (0x8002019a, "UNKNOWN_EVFID"),   
  (0x8002019b, "UNKNOWN_MBXID"),   
  (0x8002019c, "UNKNOWN_VPLID"),   
  (0x8002019d, "UNKNOWN_FPLID"),   
  (0x8002019e, "UNKNOWN_MPPID"),   
  (0x8002019f, "UNKNOWN_ALMID"),   
  (0x800201a0, "UNKNOWN_TEID"),   
  (0x800201a1, "UNKNOWN_CBID"),   
  (0x800201a2, "DORMANT"),   
  (0x800201a3, "SUSPEND"),   
  (0x800201a4, "NOT_DORMANT"),   
  (0x800201a5, "NOT_SUSPEND"),   
  (0x800201a6, "NOT_WAIT"),   
  (0x800201a7, "CAN_NOT_WAIT"),   
  (0x800201a8, "WAIT_TIMEOUT"),   
  (0x800201a9, "WAIT_CANCEL"),   
  (0x800201aa, "RELEASE_WAIT"),   
  (0x800201ab, "NOTIFY_CALLBACK"),   
  (0x800201ac, "THREAD_TERMINATED"),   
  (0x800201ad, "SEMA_ZERO"),   
  (0x800201ae, "SEMA_OVF"),   
  (0x800201af, "EVF_COND"),   
  (0x800201b0, "EVF_MULTI"),   
  (0x800201b1, "EVF_ILPAT"),   
  (0x800201b2, "MBOX_NOMSG"),   
  (0x800201b3, "MPP_FULL"),   
  (0x800201b4, "MPP_EMPTY"),   
  (0x800201b5, "WAIT_DELETE"),   
  (0x800201b6, "ILLEGAL_MEMBLOCK"),   
  (0x800201b7, "ILLEGAL_MEMSIZE"),   
  (0x800201b8, "ILLEGAL_SPADADDR"),   
  (0x800201b9, "SPAD_INUSE"),   
  (0x800201ba, "SPAD_NOT_INUSE"),   
  (0x800201bb, "ILLEGAL_TYPE"),   
  (0x800201bc, "ILLEGAL_SIZE"),   
  (0x800201bd, "ILLEGAL_COUNT"),   
  (0x800201be, "UNKNOWN_VTID"),   
  (0x800201bf, "ILLEGAL_VTID"),   
  (0x800201c0, "ILLEGAL_KTLSID"),   
  (0x800201c1, "KTLS_FULL"),   
  (0x800201c2, "KTLS_BUSY"),   
  (0x80020258, "PM_INVALID_PRIORITY"),   
  (0x80020259, "PM_INVALID_DEVNAME"),   
  (0x8002025a, "PM_UNKNOWN_DEVNAME"),   
  (0x8002025b, "PM_PMINFO_REGISTERED"),   
  (0x8002025c, "PM_PMINFO_UNREGISTERED"),   
  (0x8002025d, "PM_INVALID_MAJOR_STATE"),   
  (0x8002025e, "PM_INVALID_REQUEST"),   
  (0x8002025f, "PM_UNKNOWN_REQUEST"),   
  (0x80020260, "PM_INVALID_UNIT"),   
  (0x80020261, "PM_CANNOT_CANCEL"),   
  (0x80020262, "PM_INVALID_PMINFO"),   
  (0x80020263, "PM_INVALID_ARGUMENT"),   
  (0x80020264, "PM_ALREADY_TARGET_PWRSTATE"),   
  (0x80020265, "PM_CHANGE_PWRSTATE_FAILED"),   
  (0x80020266, "PM_CANNOT_CHANGE_DEVPWR_STATE"),   
  (0x80020267, "PM_NO_SUPPORT_DEVPWR_STATE"),   
  (0x800202bc, "DMAC_REQUEST_FAILED"),   
  (0x800202bd, "DMAC_REQUEST_DENIED"),   
  (0x800202be, "DMAC_OP_QUEUED"),   
  (0x800202bf, "DMAC_OP_NOT_QUEUED"),   
  (0x800202c0, "DMAC_OP_RUNNING"),   
  (0x800202c1, "DMAC_OP_NOT_ASSIGNED"),   
  (0x800202c2, "DMAC_OP_TIMEOUT"),   
  (0x800202c3, "DMAC_OP_FREED"),   
  (0x800202c4, "DMAC_OP_USED"),   
  (0x800202c5, "DMAC_OP_EMPTY"),   
  (0x800202c6, "DMAC_OP_ABORTED"),   
  (0x800202c7, "DMAC_OP_ERROR"),   
  (0x800202c8, "DMAC_CHANNEL_RESERVED"),   
  (0x800202c9, "DMAC_CHANNEL_EXCLUDED"),   
  (0x800202ca, "DMAC_PRIVILEGE_ADDRESS"),   
  (0x800202cb, "DMAC_NO_ENOUGHSPACE"),   
  (0x800202cc, "DMAC_CHANNEL_NOT_ASSIGNED"),   
  (0x800202cd, "DMAC_CHILD_OPERATION"),   
  (0x800202ce, "DMAC_TOO_MUCH_SIZE"),   
  (0x800202cf, "DMAC_INVALID_ARGUMENT"),   
  (0x80020320, "MFILE"),   
  (0x80020321, "NODEV"),   
  (0x80020322, "XDEV"),   
  (0x80020323, "BADF"),   
  (0x80020324, "INVAL"),   
  (0x80020325, "UNSUP"),   
  (0x80020326, "ALIAS_USED"),   
  (0x80020327, "CANNOT_MOUNT"),   
  (0x80020328, "DRIVER_DELETED"),   
  (0x80020329, "ASYNC_BUSY"),   
  (0x8002032a, "NOASYNC"),   
  (0x8002032b, "REGDEV"),   
  (0x8002032c, "NOCWD"),   
  (0x8002032d, "NAMETOOLONG"),   
  (0x800203e8, "NXIO"),   
  (0x800203e9, "IO"),   
  (0x800203ea, "NOMEM"),   
  (0x800203eb, "STDIO_NOT_OPENED"),   
  (0x8002044c, "CACHE_ALIGNMENT"),   
  (0x8002044d, "ERRORMAX"),  
  (0x80010002, "UNKNOWN")
];


fn load_module(path: &str) {
  let load_in_user_partition = SceUid(2);
  let mut option = SceKernelLMOption {
    size: size_of::<SceKernelLMOption>(),
    m_pid_text: load_in_user_partition,
    m_pid_data: load_in_user_partition,
    flags: 0,
    position: 0,
    access: 1,
    c_reserved: [0,0],
  };
  unsafe {
    let uid = sceKernelLoadModule([path,"\0"].concat().as_ptr(), 0, &mut option as *mut _);
    assert_not_error(uid.0);
    // let i32 = sceKernelStartModule(uid, 0, null_mut(), null_mut(), null_mut());
    // assert_not_error(i32);

    psp::dprintln!("loaded library with uid: {:?}", uid)
  }
}

fn assert_not_error(uid: i32) {
  let error = KERNEL_ERROR_CODES.iter().find(|e| e.0 as i32 == (uid));
  assert!(error.is_none(), "error is {:?}", error);
}


// // SceUID load_module(const char *path, int flags, int type)
// // {
// // 	SceKernelLMOption option;
// // 	SceUID mpid;

// // 	/* If the type is 0, then load the module in the kernel partition, otherwise load it
// // 	   in the user partition. */
// // 	if (type == 0) {
// // 		mpid = 1;
// // 	} else {
// // 		mpid = 2;
// // 	}

// // 	memset(&option, 0, sizeof(option));
// // 	option.size = sizeof(option);
// // 	option.mpidtext = mpid;
// // 	option.mpiddata = mpid;
// // 	option.position = 0;
// // 	option.access = 1;

// // 	return sceKernelLoadModule(path, flags, type > 0 ? &option : NULL);
// // }