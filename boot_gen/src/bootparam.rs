// Generated from linux/arch/x86/include/uapi/asm/bootparam.h
// linux commit 4b972a01a7da614b4796475f933094751a295a2f

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub const SETUP_NONE: u32 = 0;
pub const SETUP_E820_EXT: u32 = 1;
pub const SETUP_DTB: u32 = 2;
pub const SETUP_PCI: u32 = 3;
pub const SETUP_EFI: u32 = 4;
pub const SETUP_APPLE_PROPERTIES: u32 = 5;
pub const SETUP_JAILHOUSE: u32 = 6;
pub const RAMDISK_IMAGE_START_MASK: u32 = 2047;
pub const RAMDISK_PROMPT_FLAG: u32 = 32768;
pub const RAMDISK_LOAD_FLAG: u32 = 16384;
pub const LOADED_HIGH: u32 = 1;
pub const KASLR_FLAG: u32 = 2;
pub const QUIET_FLAG: u32 = 32;
pub const KEEP_SEGMENTS: u32 = 64;
pub const CAN_USE_HEAP: u32 = 128;
pub const XLF_KERNEL_64: u32 = 1;
pub const XLF_CAN_BE_LOADED_ABOVE_4G: u32 = 2;
pub const XLF_EFI_HANDOVER_32: u32 = 4;
pub const XLF_EFI_HANDOVER_64: u32 = 8;
pub const XLF_EFI_KEXEC: u32 = 16;
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const VIDEO_TYPE_MDA: u32 = 16;
pub const VIDEO_TYPE_CGA: u32 = 17;
pub const VIDEO_TYPE_EGAM: u32 = 32;
pub const VIDEO_TYPE_EGAC: u32 = 33;
pub const VIDEO_TYPE_VGAC: u32 = 34;
pub const VIDEO_TYPE_VLFB: u32 = 35;
pub const VIDEO_TYPE_PICA_S3: u32 = 48;
pub const VIDEO_TYPE_MIPS_G364: u32 = 49;
pub const VIDEO_TYPE_SGI: u32 = 51;
pub const VIDEO_TYPE_TGAC: u32 = 64;
pub const VIDEO_TYPE_SUN: u32 = 80;
pub const VIDEO_TYPE_SUNPCI: u32 = 81;
pub const VIDEO_TYPE_PMAC: u32 = 96;
pub const VIDEO_TYPE_EFI: u32 = 112;
pub const VIDEO_FLAGS_NOCURSOR: u32 = 1;
pub const VIDEO_CAPABILITY_SKIP_QUIRKS: u32 = 1;
pub const VIDEO_CAPABILITY_64BIT_BASE: u32 = 2;
pub const APM_STATE_READY: u32 = 0;
pub const APM_STATE_STANDBY: u32 = 1;
pub const APM_STATE_SUSPEND: u32 = 2;
pub const APM_STATE_OFF: u32 = 3;
pub const APM_STATE_BUSY: u32 = 4;
pub const APM_STATE_REJECT: u32 = 5;
pub const APM_STATE_OEM_SYS: u32 = 32;
pub const APM_STATE_OEM_DEV: u32 = 64;
pub const APM_STATE_DISABLE: u32 = 0;
pub const APM_STATE_ENABLE: u32 = 1;
pub const APM_STATE_DISENGAGE: u32 = 0;
pub const APM_STATE_ENGAGE: u32 = 1;
pub const APM_SYS_STANDBY: u32 = 1;
pub const APM_SYS_SUSPEND: u32 = 2;
pub const APM_NORMAL_RESUME: u32 = 3;
pub const APM_CRITICAL_RESUME: u32 = 4;
pub const APM_LOW_BATTERY: u32 = 5;
pub const APM_POWER_STATUS_CHANGE: u32 = 6;
pub const APM_UPDATE_TIME: u32 = 7;
pub const APM_CRITICAL_SUSPEND: u32 = 8;
pub const APM_USER_STANDBY: u32 = 9;
pub const APM_USER_SUSPEND: u32 = 10;
pub const APM_STANDBY_RESUME: u32 = 11;
pub const APM_CAPABILITY_CHANGE: u32 = 12;
pub const APM_USER_HIBERNATION: u32 = 13;
pub const APM_HIBERNATION_RESUME: u32 = 14;
pub const APM_SUCCESS: u32 = 0;
pub const APM_DISABLED: u32 = 1;
pub const APM_CONNECTED: u32 = 2;
pub const APM_NOT_CONNECTED: u32 = 3;
pub const APM_16_CONNECTED: u32 = 5;
pub const APM_16_UNSUPPORTED: u32 = 6;
pub const APM_32_CONNECTED: u32 = 7;
pub const APM_32_UNSUPPORTED: u32 = 8;
pub const APM_BAD_DEVICE: u32 = 9;
pub const APM_BAD_PARAM: u32 = 10;
pub const APM_NOT_ENGAGED: u32 = 11;
pub const APM_BAD_FUNCTION: u32 = 12;
pub const APM_RESUME_DISABLED: u32 = 13;
pub const APM_NO_ERROR: u32 = 83;
pub const APM_BAD_STATE: u32 = 96;
pub const APM_NO_EVENTS: u32 = 128;
pub const APM_NOT_PRESENT: u32 = 134;
pub const APM_DEVICE_BIOS: u32 = 0;
pub const APM_DEVICE_ALL: u32 = 1;
pub const APM_DEVICE_DISPLAY: u32 = 256;
pub const APM_DEVICE_STORAGE: u32 = 512;
pub const APM_DEVICE_PARALLEL: u32 = 768;
pub const APM_DEVICE_SERIAL: u32 = 1024;
pub const APM_DEVICE_NETWORK: u32 = 1280;
pub const APM_DEVICE_PCMCIA: u32 = 1536;
pub const APM_DEVICE_BATTERY: u32 = 32768;
pub const APM_DEVICE_OEM: u32 = 57344;
pub const APM_DEVICE_OLD_ALL: u32 = 65535;
pub const APM_DEVICE_CLASS: u32 = 255;
pub const APM_DEVICE_MASK: u32 = 65280;
pub const APM_MAX_BATTERIES: u32 = 2;
pub const APM_CAP_GLOBAL_STANDBY: u32 = 1;
pub const APM_CAP_GLOBAL_SUSPEND: u32 = 2;
pub const APM_CAP_RESUME_STANDBY_TIMER: u32 = 4;
pub const APM_CAP_RESUME_SUSPEND_TIMER: u32 = 8;
pub const APM_CAP_RESUME_STANDBY_RING: u32 = 16;
pub const APM_CAP_RESUME_SUSPEND_RING: u32 = 32;
pub const APM_CAP_RESUME_STANDBY_PCMCIA: u32 = 64;
pub const APM_CAP_RESUME_SUSPEND_PCMCIA: u32 = 128;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const EDDNR: u32 = 489;
pub const EDDBUF: u32 = 3328;
pub const EDDMAXNR: u32 = 6;
pub const EDDEXTSIZE: u32 = 8;
pub const EDDPARMSIZE: u32 = 74;
pub const CHECKEXTENSIONSPRESENT: u32 = 65;
pub const GETDEVICEPARAMETERS: u32 = 72;
pub const LEGACYGETDEVICEPARAMETERS: u32 = 8;
pub const EDDMAGIC1: u32 = 21930;
pub const EDDMAGIC2: u32 = 43605;
pub const READ_SECTORS: u32 = 2;
pub const EDD_MBR_SIG_OFFSET: u32 = 440;
pub const EDD_MBR_SIG_BUF: u32 = 656;
pub const EDD_MBR_SIG_MAX: u32 = 16;
pub const EDD_MBR_SIG_NR_BUF: u32 = 490;
pub const EDD_EXT_FIXED_DISK_ACCESS: u32 = 1;
pub const EDD_EXT_DEVICE_LOCKING_AND_EJECTING: u32 = 2;
pub const EDD_EXT_ENHANCED_DISK_DRIVE_SUPPORT: u32 = 4;
pub const EDD_EXT_64BIT_EXTENSIONS: u32 = 8;
pub const EDD_INFO_DMA_BOUNDARY_ERROR_TRANSPARENT: u32 = 1;
pub const EDD_INFO_GEOMETRY_VALID: u32 = 2;
pub const EDD_INFO_REMOVABLE: u32 = 4;
pub const EDD_INFO_WRITE_VERIFY: u32 = 8;
pub const EDD_INFO_MEDIA_CHANGE_NOTIFICATION: u32 = 16;
pub const EDD_INFO_LOCKABLE: u32 = 32;
pub const EDD_INFO_NO_MEDIA_PRESENT: u32 = 64;
pub const EDD_INFO_USE_INT13_FN50: u32 = 128;
pub const E820_MAX_ENTRIES_ZEROPAGE: u32 = 128;
pub const JAILHOUSE_SETUP_REQUIRED_VERSION: u32 = 1;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fsid_t>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = ::std::os::raw::c_uint;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct screen_info {
    pub orig_x: __u8,
    pub orig_y: __u8,
    pub ext_mem_k: __u16,
    pub orig_video_page: __u16,
    pub orig_video_mode: __u8,
    pub orig_video_cols: __u8,
    pub flags: __u8,
    pub unused2: __u8,
    pub orig_video_ega_bx: __u16,
    pub unused3: __u16,
    pub orig_video_lines: __u8,
    pub orig_video_isVGA: __u8,
    pub orig_video_points: __u16,
    pub lfb_width: __u16,
    pub lfb_height: __u16,
    pub lfb_depth: __u16,
    pub lfb_base: __u32,
    pub lfb_size: __u32,
    pub cl_magic: __u16,
    pub cl_offset: __u16,
    pub lfb_linelength: __u16,
    pub red_size: __u8,
    pub red_pos: __u8,
    pub green_size: __u8,
    pub green_pos: __u8,
    pub blue_size: __u8,
    pub blue_pos: __u8,
    pub rsvd_size: __u8,
    pub rsvd_pos: __u8,
    pub vesapm_seg: __u16,
    pub vesapm_off: __u16,
    pub pages: __u16,
    pub vesa_attributes: __u16,
    pub capabilities: __u32,
    pub ext_lfb_base: __u32,
    pub _reserved: [__u8; 2usize],
}
#[test]
fn bindgen_test_layout_screen_info() {
    assert_eq!(
        ::std::mem::size_of::<screen_info>(),
        64usize,
        concat!("Size of: ", stringify!(screen_info))
    );
    assert_eq!(
        ::std::mem::align_of::<screen_info>(),
        1usize,
        concat!("Alignment of ", stringify!(screen_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_y as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).ext_mem_k as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(ext_mem_k)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_page as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_page)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_mode as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_cols as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_cols)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).unused2 as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(unused2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_ega_bx as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_ega_bx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).unused3 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(unused3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_lines as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_lines)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_isVGA as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_isVGA)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).orig_video_points as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(orig_video_points)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).lfb_width as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(lfb_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).lfb_height as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(lfb_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).lfb_depth as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(lfb_depth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).lfb_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(lfb_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).lfb_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(lfb_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).cl_magic as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(cl_magic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).cl_offset as *const _ as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(cl_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).lfb_linelength as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(lfb_linelength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).red_size as *const _ as usize },
        38usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(red_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).red_pos as *const _ as usize },
        39usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(red_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).green_size as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(green_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).green_pos as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(green_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).blue_size as *const _ as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(blue_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).blue_pos as *const _ as usize },
        43usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(blue_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).rsvd_size as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(rsvd_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).rsvd_pos as *const _ as usize },
        45usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(rsvd_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).vesapm_seg as *const _ as usize },
        46usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(vesapm_seg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).vesapm_off as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(vesapm_off)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).pages as *const _ as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(pages)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).vesa_attributes as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(vesa_attributes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).capabilities as *const _ as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(capabilities)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>())).ext_lfb_base as *const _ as usize },
        58usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(ext_lfb_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<screen_info>()))._reserved as *const _ as usize },
        62usize,
        concat!(
            "Offset of field: ",
            stringify!(screen_info),
            "::",
            stringify!(_reserved)
        )
    );
}
pub type apm_event_t = ::std::os::raw::c_ushort;
pub type apm_eventinfo_t = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct apm_bios_info {
    pub version: __u16,
    pub cseg: __u16,
    pub offset: __u32,
    pub cseg_16: __u16,
    pub dseg: __u16,
    pub flags: __u16,
    pub cseg_len: __u16,
    pub cseg_16_len: __u16,
    pub dseg_len: __u16,
}
#[test]
fn bindgen_test_layout_apm_bios_info() {
    assert_eq!(
        ::std::mem::size_of::<apm_bios_info>(),
        20usize,
        concat!("Size of: ", stringify!(apm_bios_info))
    );
    assert_eq!(
        ::std::mem::align_of::<apm_bios_info>(),
        4usize,
        concat!("Alignment of ", stringify!(apm_bios_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).cseg as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(cseg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).cseg_16 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(cseg_16)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).dseg as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(dseg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).cseg_len as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(cseg_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).cseg_16_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(cseg_16_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<apm_bios_info>())).dseg_len as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(apm_bios_info),
            "::",
            stringify!(dseg_len)
        )
    );
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct edd_device_params {
    pub length: __u16,
    pub info_flags: __u16,
    pub num_default_cylinders: __u32,
    pub num_default_heads: __u32,
    pub sectors_per_track: __u32,
    pub number_of_sectors: __u64,
    pub bytes_per_sector: __u16,
    pub dpte_ptr: __u32,
    pub key: __u16,
    pub device_path_info_length: __u8,
    pub reserved2: __u8,
    pub reserved3: __u16,
    pub host_bus_type: [__u8; 4usize],
    pub interface_type: [__u8; 8usize],
    pub interface_path: edd_device_params__bindgen_ty_1,
    pub device_path: edd_device_params__bindgen_ty_2,
    pub reserved4: __u8,
    pub checksum: __u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union edd_device_params__bindgen_ty_1 {
    pub isa: edd_device_params__bindgen_ty_1__bindgen_ty_1,
    pub pci: edd_device_params__bindgen_ty_1__bindgen_ty_2,
    pub ibnd: edd_device_params__bindgen_ty_1__bindgen_ty_3,
    pub xprs: edd_device_params__bindgen_ty_1__bindgen_ty_4,
    pub htpt: edd_device_params__bindgen_ty_1__bindgen_ty_5,
    pub unknown: edd_device_params__bindgen_ty_1__bindgen_ty_6,
    _bindgen_union_align: [u8; 8usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_1 {
    pub base_address: __u16,
    pub reserved1: __u16,
    pub reserved2: __u32,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_1>())).base_address
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(base_address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_1>())).reserved1
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_1>())).reserved2
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(reserved2)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_2 {
    pub bus: __u8,
    pub slot: __u8,
    pub function: __u8,
    pub channel: __u8,
    pub reserved: __u32,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1__bindgen_ty_2>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_2>())).bus
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(bus)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_2>())).slot
                as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(slot)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_2>())).function
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_2>())).channel
                as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(channel)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_2>())).reserved
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_3 {
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1__bindgen_ty_3>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_3>())).reserved
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_4 {
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1__bindgen_ty_4>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_4>())).reserved
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_5 {
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1__bindgen_ty_5>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1__bindgen_ty_5>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_5>())).reserved
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_6 {
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1__bindgen_ty_6() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1__bindgen_ty_6>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1__bindgen_ty_6>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1__bindgen_ty_6>())).reserved
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(reserved)
        )
    );
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(edd_device_params__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(edd_device_params__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1>())).isa as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1),
            "::",
            stringify!(isa)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1>())).pci as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1),
            "::",
            stringify!(pci)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1>())).ibnd as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1),
            "::",
            stringify!(ibnd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1>())).xprs as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1),
            "::",
            stringify!(xprs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1>())).htpt as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1),
            "::",
            stringify!(htpt)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_1>())).unknown as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_1),
            "::",
            stringify!(unknown)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union edd_device_params__bindgen_ty_2 {
    pub ata: edd_device_params__bindgen_ty_2__bindgen_ty_1,
    pub atapi: edd_device_params__bindgen_ty_2__bindgen_ty_2,
    pub scsi: edd_device_params__bindgen_ty_2__bindgen_ty_3,
    pub usb: edd_device_params__bindgen_ty_2__bindgen_ty_4,
    pub i1394: edd_device_params__bindgen_ty_2__bindgen_ty_5,
    pub fibre: edd_device_params__bindgen_ty_2__bindgen_ty_6,
    pub i2o: edd_device_params__bindgen_ty_2__bindgen_ty_7,
    pub raid: edd_device_params__bindgen_ty_2__bindgen_ty_8,
    pub sata: edd_device_params__bindgen_ty_2__bindgen_ty_9,
    pub unknown: edd_device_params__bindgen_ty_2__bindgen_ty_10,
    _bindgen_union_align: [u8; 16usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_1 {
    pub device: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub reserved4: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_1>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_1>())).device
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(device)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_1>())).reserved1
                as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_1>())).reserved2
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(reserved2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_1>())).reserved3
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(reserved3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_1>())).reserved4
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(reserved4)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_2 {
    pub device: __u8,
    pub lun: __u8,
    pub reserved1: __u8,
    pub reserved2: __u8,
    pub reserved3: __u32,
    pub reserved4: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_2>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_2>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_2>())).device
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2),
            "::",
            stringify!(device)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_2>())).lun
                as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_2>())).reserved1
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_2>())).reserved2
                as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2),
            "::",
            stringify!(reserved2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_2>())).reserved3
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2),
            "::",
            stringify!(reserved3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_2>())).reserved4
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_2),
            "::",
            stringify!(reserved4)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_3 {
    pub id: __u16,
    pub lun: __u64,
    pub reserved1: __u16,
    pub reserved2: __u32,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_3>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_3>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_3>())).id as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_3),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_3>())).lun
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_3),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_3>())).reserved1
                as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_3),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_3>())).reserved2
                as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_3),
            "::",
            stringify!(reserved2)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_4 {
    pub serial_number: __u64,
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_4>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_4>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_4>())).serial_number
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_4),
            "::",
            stringify!(serial_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_4>())).reserved
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_4),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_5 {
    pub eui: __u64,
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_5>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_5>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_5>())).eui
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_5),
            "::",
            stringify!(eui)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_5>())).reserved
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_5),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_6 {
    pub wwid: __u64,
    pub lun: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_6() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_6>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_6)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_6>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_6>())).wwid
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_6),
            "::",
            stringify!(wwid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_6>())).lun
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_6),
            "::",
            stringify!(lun)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_7 {
    pub identity_tag: __u64,
    pub reserved: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_7() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_7>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_7)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_7>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_7>())).identity_tag
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_7),
            "::",
            stringify!(identity_tag)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_7>())).reserved
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_7),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_8 {
    pub array_number: __u32,
    pub reserved1: __u32,
    pub reserved2: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_8() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_8>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_8)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_8>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_8>())).array_number
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_8),
            "::",
            stringify!(array_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_8>())).reserved1
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_8),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_8>())).reserved2
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_8),
            "::",
            stringify!(reserved2)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_9 {
    pub device: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub reserved4: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_9() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_9>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_9>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_9>())).device
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9),
            "::",
            stringify!(device)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_9>())).reserved1
                as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_9>())).reserved2
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9),
            "::",
            stringify!(reserved2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_9>())).reserved3
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9),
            "::",
            stringify!(reserved3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_9>())).reserved4
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_9),
            "::",
            stringify!(reserved4)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_10 {
    pub reserved1: __u64,
    pub reserved2: __u64,
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2__bindgen_ty_10() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2__bindgen_ty_10>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_10)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2__bindgen_ty_10>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_10>())).reserved1
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_10),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2__bindgen_ty_10>())).reserved2
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2__bindgen_ty_10),
            "::",
            stringify!(reserved2)
        )
    );
}
#[test]
fn bindgen_test_layout_edd_device_params__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params__bindgen_ty_2>(),
        16usize,
        concat!("Size of: ", stringify!(edd_device_params__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(edd_device_params__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).ata as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(ata)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).atapi as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(atapi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).scsi as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(scsi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).usb as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(usb)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).i1394 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(i1394)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).fibre as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(fibre)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).i2o as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(i2o)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).raid as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(raid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).sata as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(sata)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params__bindgen_ty_2>())).unknown as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params__bindgen_ty_2),
            "::",
            stringify!(unknown)
        )
    );
}
#[test]
fn bindgen_test_layout_edd_device_params() {
    assert_eq!(
        ::std::mem::size_of::<edd_device_params>(),
        74usize,
        concat!("Size of: ", stringify!(edd_device_params))
    );
    assert_eq!(
        ::std::mem::align_of::<edd_device_params>(),
        1usize,
        concat!("Alignment of ", stringify!(edd_device_params))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).info_flags as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(info_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).num_default_cylinders as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(num_default_cylinders)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).num_default_heads as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(num_default_heads)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).sectors_per_track as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(sectors_per_track)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).number_of_sectors as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(number_of_sectors)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).bytes_per_sector as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(bytes_per_sector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).dpte_ptr as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(dpte_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).key as *const _ as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).device_path_info_length as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(device_path_info_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).reserved2 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(reserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).reserved3 as *const _ as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(reserved3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).host_bus_type as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(host_bus_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).interface_type as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(interface_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_device_params>())).interface_path as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(interface_path)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).device_path as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(device_path)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).reserved4 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(reserved4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_device_params>())).checksum as *const _ as usize },
        73usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_device_params),
            "::",
            stringify!(checksum)
        )
    );
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct edd_info {
    pub device: __u8,
    pub version: __u8,
    pub interface_support: __u16,
    pub legacy_max_cylinder: __u16,
    pub legacy_max_head: __u8,
    pub legacy_sectors_per_track: __u8,
    pub params: edd_device_params,
}
#[test]
fn bindgen_test_layout_edd_info() {
    assert_eq!(
        ::std::mem::size_of::<edd_info>(),
        82usize,
        concat!("Size of: ", stringify!(edd_info))
    );
    assert_eq!(
        ::std::mem::align_of::<edd_info>(),
        1usize,
        concat!("Alignment of ", stringify!(edd_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_info>())).device as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(device)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_info>())).version as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_info>())).interface_support as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(interface_support)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_info>())).legacy_max_cylinder as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(legacy_max_cylinder)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_info>())).legacy_max_head as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(legacy_max_head)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<edd_info>())).legacy_sectors_per_track as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(legacy_sectors_per_track)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd_info>())).params as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(edd_info),
            "::",
            stringify!(params)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edd {
    pub mbr_signature: [::std::os::raw::c_uint; 16usize],
    pub edd_info: [edd_info; 6usize],
    pub mbr_signature_nr: ::std::os::raw::c_uchar,
    pub edd_info_nr: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_edd() {
    assert_eq!(
        ::std::mem::size_of::<edd>(),
        560usize,
        concat!("Size of: ", stringify!(edd))
    );
    assert_eq!(
        ::std::mem::align_of::<edd>(),
        4usize,
        concat!("Alignment of ", stringify!(edd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd>())).mbr_signature as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edd),
            "::",
            stringify!(mbr_signature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd>())).edd_info as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(edd),
            "::",
            stringify!(edd_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd>())).mbr_signature_nr as *const _ as usize },
        556usize,
        concat!(
            "Offset of field: ",
            stringify!(edd),
            "::",
            stringify!(mbr_signature_nr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edd>())).edd_info_nr as *const _ as usize },
        557usize,
        concat!(
            "Offset of field: ",
            stringify!(edd),
            "::",
            stringify!(edd_info_nr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ist_info {
    pub signature: __u32,
    pub command: __u32,
    pub event: __u32,
    pub perf_level: __u32,
}
#[test]
fn bindgen_test_layout_ist_info() {
    assert_eq!(
        ::std::mem::size_of::<ist_info>(),
        16usize,
        concat!("Size of: ", stringify!(ist_info))
    );
    assert_eq!(
        ::std::mem::align_of::<ist_info>(),
        4usize,
        concat!("Alignment of ", stringify!(ist_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ist_info>())).signature as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ist_info),
            "::",
            stringify!(signature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ist_info>())).command as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ist_info),
            "::",
            stringify!(command)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ist_info>())).event as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ist_info),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ist_info>())).perf_level as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ist_info),
            "::",
            stringify!(perf_level)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edid_info {
    pub dummy: [::std::os::raw::c_uchar; 128usize],
}
#[test]
fn bindgen_test_layout_edid_info() {
    assert_eq!(
        ::std::mem::size_of::<edid_info>(),
        128usize,
        concat!("Size of: ", stringify!(edid_info))
    );
    assert_eq!(
        ::std::mem::align_of::<edid_info>(),
        1usize,
        concat!("Alignment of ", stringify!(edid_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<edid_info>())).dummy as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(edid_info),
            "::",
            stringify!(dummy)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct setup_data {
    pub next: __u64,
    pub type_: __u32,
    pub len: __u32,
    pub data: __IncompleteArrayField<__u8>,
}
#[test]
fn bindgen_test_layout_setup_data() {
    assert_eq!(
        ::std::mem::size_of::<setup_data>(),
        16usize,
        concat!("Size of: ", stringify!(setup_data))
    );
    assert_eq!(
        ::std::mem::align_of::<setup_data>(),
        8usize,
        concat!("Alignment of ", stringify!(setup_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_data>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_data),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_data>())).type_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_data),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_data>())).len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_data),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_data>())).data as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_data),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct setup_header {
    pub setup_sects: __u8,
    pub root_flags: __u16,
    pub syssize: __u32,
    pub ram_size: __u16,
    pub vid_mode: __u16,
    pub root_dev: __u16,
    pub boot_flag: __u16,
    pub jump: __u16,
    pub header: __u32,
    pub version: __u16,
    pub realmode_swtch: __u32,
    pub start_sys_seg: __u16,
    pub kernel_version: __u16,
    pub type_of_loader: __u8,
    pub loadflags: __u8,
    pub setup_move_size: __u16,
    pub code32_start: __u32,
    pub ramdisk_image: __u32,
    pub ramdisk_size: __u32,
    pub bootsect_kludge: __u32,
    pub heap_end_ptr: __u16,
    pub ext_loader_ver: __u8,
    pub ext_loader_type: __u8,
    pub cmd_line_ptr: __u32,
    pub initrd_addr_max: __u32,
    pub kernel_alignment: __u32,
    pub relocatable_kernel: __u8,
    pub min_alignment: __u8,
    pub xloadflags: __u16,
    pub cmdline_size: __u32,
    pub hardware_subarch: __u32,
    pub hardware_subarch_data: __u64,
    pub payload_offset: __u32,
    pub payload_length: __u32,
    pub setup_data: __u64,
    pub pref_address: __u64,
    pub init_size: __u32,
    pub handover_offset: __u32,
}
#[test]
fn bindgen_test_layout_setup_header() {
    assert_eq!(
        ::std::mem::size_of::<setup_header>(),
        119usize,
        concat!("Size of: ", stringify!(setup_header))
    );
    assert_eq!(
        ::std::mem::align_of::<setup_header>(),
        1usize,
        concat!("Alignment of ", stringify!(setup_header))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).setup_sects as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(setup_sects)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).root_flags as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(root_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).syssize as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(syssize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).ram_size as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(ram_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).vid_mode as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(vid_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).root_dev as *const _ as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(root_dev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).boot_flag as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(boot_flag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).jump as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(jump)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).header as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(header)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).version as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).realmode_swtch as *const _ as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(realmode_swtch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).start_sys_seg as *const _ as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(start_sys_seg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).kernel_version as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(kernel_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).type_of_loader as *const _ as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(type_of_loader)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).loadflags as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(loadflags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).setup_move_size as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(setup_move_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).code32_start as *const _ as usize },
        35usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(code32_start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).ramdisk_image as *const _ as usize },
        39usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(ramdisk_image)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).ramdisk_size as *const _ as usize },
        43usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(ramdisk_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).bootsect_kludge as *const _ as usize },
        47usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(bootsect_kludge)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).heap_end_ptr as *const _ as usize },
        51usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(heap_end_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).ext_loader_ver as *const _ as usize },
        53usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(ext_loader_ver)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).ext_loader_type as *const _ as usize },
        54usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(ext_loader_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).cmd_line_ptr as *const _ as usize },
        55usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(cmd_line_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).initrd_addr_max as *const _ as usize },
        59usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(initrd_addr_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).kernel_alignment as *const _ as usize },
        63usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(kernel_alignment)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).relocatable_kernel as *const _ as usize },
        67usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(relocatable_kernel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).min_alignment as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(min_alignment)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).xloadflags as *const _ as usize },
        69usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(xloadflags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).cmdline_size as *const _ as usize },
        71usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(cmdline_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).hardware_subarch as *const _ as usize },
        75usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(hardware_subarch)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<setup_header>())).hardware_subarch_data as *const _ as usize
        },
        79usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(hardware_subarch_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).payload_offset as *const _ as usize },
        87usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(payload_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).payload_length as *const _ as usize },
        91usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(payload_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).setup_data as *const _ as usize },
        95usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(setup_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).pref_address as *const _ as usize },
        103usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(pref_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).init_size as *const _ as usize },
        111usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(init_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setup_header>())).handover_offset as *const _ as usize },
        115usize,
        concat!(
            "Offset of field: ",
            stringify!(setup_header),
            "::",
            stringify!(handover_offset)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sys_desc_table {
    pub length: __u16,
    pub table: [__u8; 14usize],
}
#[test]
fn bindgen_test_layout_sys_desc_table() {
    assert_eq!(
        ::std::mem::size_of::<sys_desc_table>(),
        16usize,
        concat!("Size of: ", stringify!(sys_desc_table))
    );
    assert_eq!(
        ::std::mem::align_of::<sys_desc_table>(),
        2usize,
        concat!("Alignment of ", stringify!(sys_desc_table))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sys_desc_table>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sys_desc_table),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sys_desc_table>())).table as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sys_desc_table),
            "::",
            stringify!(table)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct olpc_ofw_header {
    pub ofw_magic: __u32,
    pub ofw_version: __u32,
    pub cif_handler: __u32,
    pub irq_desc_table: __u32,
}
#[test]
fn bindgen_test_layout_olpc_ofw_header() {
    assert_eq!(
        ::std::mem::size_of::<olpc_ofw_header>(),
        16usize,
        concat!("Size of: ", stringify!(olpc_ofw_header))
    );
    assert_eq!(
        ::std::mem::align_of::<olpc_ofw_header>(),
        1usize,
        concat!("Alignment of ", stringify!(olpc_ofw_header))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<olpc_ofw_header>())).ofw_magic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(olpc_ofw_header),
            "::",
            stringify!(ofw_magic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<olpc_ofw_header>())).ofw_version as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(olpc_ofw_header),
            "::",
            stringify!(ofw_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<olpc_ofw_header>())).cif_handler as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(olpc_ofw_header),
            "::",
            stringify!(cif_handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<olpc_ofw_header>())).irq_desc_table as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(olpc_ofw_header),
            "::",
            stringify!(irq_desc_table)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct efi_info {
    pub efi_loader_signature: __u32,
    pub efi_systab: __u32,
    pub efi_memdesc_size: __u32,
    pub efi_memdesc_version: __u32,
    pub efi_memmap: __u32,
    pub efi_memmap_size: __u32,
    pub efi_systab_hi: __u32,
    pub efi_memmap_hi: __u32,
}
#[test]
fn bindgen_test_layout_efi_info() {
    assert_eq!(
        ::std::mem::size_of::<efi_info>(),
        32usize,
        concat!("Size of: ", stringify!(efi_info))
    );
    assert_eq!(
        ::std::mem::align_of::<efi_info>(),
        4usize,
        concat!("Alignment of ", stringify!(efi_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_loader_signature as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_loader_signature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_systab as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_systab)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_memdesc_size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_memdesc_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_memdesc_version as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_memdesc_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_memmap as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_memmap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_memmap_size as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_memmap_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_systab_hi as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_systab_hi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<efi_info>())).efi_memmap_hi as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(efi_info),
            "::",
            stringify!(efi_memmap_hi)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct boot_e820_entry {
    pub addr: __u64,
    pub size: __u64,
    pub type_: __u32,
}
#[test]
fn bindgen_test_layout_boot_e820_entry() {
    assert_eq!(
        ::std::mem::size_of::<boot_e820_entry>(),
        20usize,
        concat!("Size of: ", stringify!(boot_e820_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<boot_e820_entry>(),
        1usize,
        concat!("Alignment of ", stringify!(boot_e820_entry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_e820_entry>())).addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_e820_entry),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_e820_entry>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_e820_entry),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_e820_entry>())).type_ as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_e820_entry),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct jailhouse_setup_data {
    pub version: __u16,
    pub compatible_version: __u16,
    pub pm_timer_address: __u16,
    pub num_cpus: __u16,
    pub pci_mmconfig_base: __u64,
    pub tsc_khz: __u32,
    pub apic_khz: __u32,
    pub standard_ioapic: __u8,
    pub cpu_ids: [__u8; 255usize],
}
#[test]
fn bindgen_test_layout_jailhouse_setup_data() {
    assert_eq!(
        ::std::mem::size_of::<jailhouse_setup_data>(),
        280usize,
        concat!("Size of: ", stringify!(jailhouse_setup_data))
    );
    assert_eq!(
        ::std::mem::align_of::<jailhouse_setup_data>(),
        1usize,
        concat!("Alignment of ", stringify!(jailhouse_setup_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jailhouse_setup_data>())).version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jailhouse_setup_data>())).compatible_version as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(compatible_version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jailhouse_setup_data>())).pm_timer_address as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(pm_timer_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jailhouse_setup_data>())).num_cpus as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(num_cpus)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jailhouse_setup_data>())).pci_mmconfig_base as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(pci_mmconfig_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jailhouse_setup_data>())).tsc_khz as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(tsc_khz)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jailhouse_setup_data>())).apic_khz as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(apic_khz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jailhouse_setup_data>())).standard_ioapic as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(standard_ioapic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jailhouse_setup_data>())).cpu_ids as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(jailhouse_setup_data),
            "::",
            stringify!(cpu_ids)
        )
    );
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct boot_params {
    pub screen_info: screen_info,
    pub apm_bios_info: apm_bios_info,
    pub _pad2: [__u8; 4usize],
    pub tboot_addr: __u64,
    pub ist_info: ist_info,
    pub acpi_rsdp_addr: __u64,
    pub _pad3: [__u8; 8usize],
    pub hd0_info: [__u8; 16usize],
    pub hd1_info: [__u8; 16usize],
    pub sys_desc_table: sys_desc_table,
    pub olpc_ofw_header: olpc_ofw_header,
    pub ext_ramdisk_image: __u32,
    pub ext_ramdisk_size: __u32,
    pub ext_cmd_line_ptr: __u32,
    pub _pad4: [__u8; 116usize],
    pub edid_info: edid_info,
    pub efi_info: efi_info,
    pub alt_mem_k: __u32,
    pub scratch: __u32,
    pub e820_entries: __u8,
    pub eddbuf_entries: __u8,
    pub edd_mbr_sig_buf_entries: __u8,
    pub kbd_status: __u8,
    pub secure_boot: __u8,
    pub _pad5: [__u8; 2usize],
    pub sentinel: __u8,
    pub _pad6: [__u8; 1usize],
    pub hdr: setup_header,
    pub _pad7: [__u8; 40usize],
    pub edd_mbr_sig_buffer: [__u32; 16usize],
    pub e820_table: [boot_e820_entry; 128usize],
    pub _pad8: [__u8; 48usize],
    pub eddbuf: [edd_info; 6usize],
    pub _pad9: [__u8; 276usize],
}
#[test]
fn bindgen_test_layout_boot_params() {
    assert_eq!(
        ::std::mem::size_of::<boot_params>(),
        4096usize,
        concat!("Size of: ", stringify!(boot_params))
    );
    assert_eq!(
        ::std::mem::align_of::<boot_params>(),
        1usize,
        concat!("Alignment of ", stringify!(boot_params))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).screen_info as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(screen_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).apm_bios_info as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(apm_bios_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad2 as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).tboot_addr as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(tboot_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).ist_info as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(ist_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).acpi_rsdp_addr as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(acpi_rsdp_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad3 as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).hd0_info as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(hd0_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).hd1_info as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(hd1_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).sys_desc_table as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(sys_desc_table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).olpc_ofw_header as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(olpc_ofw_header)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).ext_ramdisk_image as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(ext_ramdisk_image)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).ext_ramdisk_size as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(ext_ramdisk_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).ext_cmd_line_ptr as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(ext_cmd_line_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad4 as *const _ as usize },
        204usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).edid_info as *const _ as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(edid_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).efi_info as *const _ as usize },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(efi_info)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).alt_mem_k as *const _ as usize },
        480usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(alt_mem_k)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).scratch as *const _ as usize },
        484usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(scratch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).e820_entries as *const _ as usize },
        488usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(e820_entries)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).eddbuf_entries as *const _ as usize },
        489usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(eddbuf_entries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<boot_params>())).edd_mbr_sig_buf_entries as *const _ as usize
        },
        490usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(edd_mbr_sig_buf_entries)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).kbd_status as *const _ as usize },
        491usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(kbd_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).secure_boot as *const _ as usize },
        492usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(secure_boot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad5 as *const _ as usize },
        493usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad5)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).sentinel as *const _ as usize },
        495usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(sentinel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad6 as *const _ as usize },
        496usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).hdr as *const _ as usize },
        497usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(hdr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad7 as *const _ as usize },
        616usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad7)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).edd_mbr_sig_buffer as *const _ as usize },
        656usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(edd_mbr_sig_buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).e820_table as *const _ as usize },
        720usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(e820_table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad8 as *const _ as usize },
        3280usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>())).eddbuf as *const _ as usize },
        3328usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(eddbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<boot_params>()))._pad9 as *const _ as usize },
        3820usize,
        concat!(
            "Offset of field: ",
            stringify!(boot_params),
            "::",
            stringify!(_pad9)
        )
    );
}
pub const x86_hardware_subarch_X86_SUBARCH_PC: x86_hardware_subarch = 0;
pub const x86_hardware_subarch_X86_SUBARCH_LGUEST: x86_hardware_subarch = 1;
pub const x86_hardware_subarch_X86_SUBARCH_XEN: x86_hardware_subarch = 2;
pub const x86_hardware_subarch_X86_SUBARCH_INTEL_MID: x86_hardware_subarch = 3;
pub const x86_hardware_subarch_X86_SUBARCH_CE4100: x86_hardware_subarch = 4;
pub const x86_hardware_subarch_X86_NR_SUBARCHS: x86_hardware_subarch = 5;
#[doc = " enum x86_hardware_subarch - x86 hardware subarchitecture"]
#[doc = ""]
#[doc = " The x86 hardware_subarch and hardware_subarch_data were added as of the x86"]
#[doc = " boot protocol 2.07 to help distinguish and support custom x86 boot"]
#[doc = " sequences. This enum represents accepted values for the x86"]
#[doc = " hardware_subarch.  Custom x86 boot sequences (not X86_SUBARCH_PC) do not"]
#[doc = " have or simply *cannot* make use of natural stubs like BIOS or EFI, the"]
#[doc = " hardware_subarch can be used on the Linux entry path to revector to a"]
#[doc = " subarchitecture stub when needed. This subarchitecture stub can be used to"]
#[doc = " set up Linux boot parameters or for special care to account for nonstandard"]
#[doc = " handling of page tables."]
#[doc = ""]
#[doc = " These enums should only ever be used by x86 code, and the code that uses"]
#[doc = " it should be well contained and compartamentalized."]
#[doc = ""]
#[doc = " KVM and Xen HVM do not have a subarch as these are expected to follow"]
#[doc = " standard x86 boot entries. If there is a genuine need for \"hypervisor\" type"]
#[doc = " that should be considered separately in the future. Future guest types"]
#[doc = " should seriously consider working with standard x86 boot stubs such as"]
#[doc = " the BIOS or EFI boot stubs."]
#[doc = ""]
#[doc = " WARNING: this enum is only used for legacy hacks, for platform features that"]
#[doc = "\t    are not easily enumerated or discoverable. You should not ever use"]
#[doc = "\t    this for new features."]
#[doc = ""]
#[doc = " @X86_SUBARCH_PC: Should be used if the hardware is enumerable using standard"]
#[doc = "\tPC mechanisms (PCI, ACPI) and doesn't need a special boot flow."]
#[doc = " @X86_SUBARCH_LGUEST: Used for x86 hypervisor demo, lguest, deprecated"]
#[doc = " @X86_SUBARCH_XEN: Used for Xen guest types which follow the PV boot path,"]
#[doc = " \twhich start at asm startup_xen() entry point and later jump to the C"]
#[doc = " \txen_start_kernel() entry point. Both domU and dom0 type of guests are"]
#[doc = " \tcurrently supportd through this PV boot path."]
#[doc = " @X86_SUBARCH_INTEL_MID: Used for Intel MID (Mobile Internet Device) platform"]
#[doc = "\tsystems which do not have the PCI legacy interfaces."]
#[doc = " @X86_SUBARCH_CE4100: Used for Intel CE media processor (CE4100) SoC for"]
#[doc = " \tfor settop boxes and media devices, the use of a subarch for CE4100"]
#[doc = " \tis more of a hack..."]
pub type x86_hardware_subarch = u32;