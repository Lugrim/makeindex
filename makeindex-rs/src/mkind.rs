#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use libc::*;
use libc_stdhandle::*;

use std::ffi::{CStr, CString};

use clap::{ArgAction, Parser};

#[derive(Parser, Clone)]
pub struct CliArguments {
    /// Use stdin intead of input files
    #[arg(short = 'i')]
    use_stdin: bool,

    /// Enable letter ordering
    #[arg(short = 'l')]
    _letter_ordering: bool,

    /// Supress progress message (quiet mode)
    #[arg(short = 'q', action=ArgAction::SetFalse)]
    _verbose: bool,

    /// Disable range merge
    #[arg(short = 'r', action=ArgAction::SetFalse)]
    _merge_page: bool,

    /// Compress blanks
    #[arg(short = 'c')]
    _compress_blanks: bool,

    /// Enable german sort
    #[arg(short = 'g')]
    _german_sort: bool,

    /// Style file
    #[arg(short = 's')]
    style: Option<String>,

    /// Output file
    #[arg(short = 'o')]
    output: Option<String>,

    /// Transcript file
    #[arg(short = 't')]
    transcript: Option<String>,

    /// Initial page
    #[arg(short = 'p', value_parser = parse_init_page)]
    _init_page: Option<InitialPage>,

    /// Input .idx files
    #[arg(trailing_var_arg = true)]
    input_files: Vec<String>,
}

#[derive(Clone, Copy)]
enum InitialPage {
    Even,
    Odd,
    Any,
    Number(usize),
}

impl std::fmt::Display for InitialPage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            InitialPage::Even => write!(f, "even"),
            InitialPage::Odd => write!(f, "odd"),
            InitialPage::Any => write!(f, "any"),
            InitialPage::Number(n) => write!(f, "{n}"),
        }
    }
}

fn parse_init_page(arg: &str) -> Result<InitialPage, std::num::ParseIntError> {
    match arg {
        "even" => Ok(InitialPage::Even),
        "odd" => Ok(InitialPage::Odd),
        "any" => Ok(InitialPage::Any),
        nbr => str::parse::<usize>(nbr).map(InitialPage::Number),
    }
}

extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut idx_quote: libc::c_char;
    static mut head: NODE_PTR;
    fn gen_ind();
    fn scan_idx();
    fn scan_sty();
    fn sort_idx();
}
pub type _IO_lock_t = ();
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KFIELD {
    pub sf: [*mut libc::c_char; 3],
    pub af: [*mut libc::c_char; 3],
    pub group: i32,
    pub lpg: [libc::c_char; 16],
    pub npg: [i16; 10],
    pub count: i16,
    pub type_0: i16,
    pub encap: *mut libc::c_char,
    pub fn_0: *mut libc::c_char,
    pub lc: i32,
}
pub type FIELD = KFIELD;
pub type FIELD_PTR = *mut KFIELD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KNODE {
    pub data: FIELD,
    pub next: *mut KNODE,
}
pub type NODE_PTR = *mut KNODE;
#[no_mangle]
pub static mut letter_ordering: i32 = 0;
#[no_mangle]
pub static mut compress_blanks: i32 = 0;
#[no_mangle]
pub static mut merge_page: i32 = 1;
#[no_mangle]
pub static mut init_page: i32 = 0;
#[no_mangle]
pub static mut even_odd: i32 = -(1);
#[no_mangle]
pub static mut verbose: bool = true;
#[no_mangle]
pub static mut german_sort: i32 = 0;
#[no_mangle]
pub static mut fn_no: i32 = -(1);
#[no_mangle]
pub static mut idx_dot: i32 = 1;
#[no_mangle]
pub static mut idx_tt: i32 = 0;
#[no_mangle]
pub static mut idx_et: i32 = 0;
#[no_mangle]
pub static mut idx_gt: i32 = 0;
#[no_mangle]
pub static mut idx_key: *mut FIELD_PTR = 0 as *const FIELD_PTR as *mut FIELD_PTR;
#[no_mangle]
pub static mut log_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut sty_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut idx_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut ind_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut ilg_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut pgm_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut sty_fn: [libc::c_char; 72] = [0; 72];
#[no_mangle]
pub static mut idx_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ind: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ind_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ilg: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ilg_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut pageno: [libc::c_char; 16] = [0; 16];
static mut log_fn: [libc::c_char; 256] = [0; 256];
static mut base: [libc::c_char; 256] = [0; 256];
static mut need_version: bool = true;

#[no_mangle]
// TODO: use a proper output library like log or tracing
// TODO: return a Result<i32, _> proper error management
pub fn makeindex_main(mut args: CliArguments) -> i32 {
    let mut fns = [std::ptr::null_mut::<libc::c_char>(); 1024];
    // let mut ap = std::ptr::null_mut::<libc::c_char>(); // Was used to manually parse arguments
    let mut use_stdin = args.use_stdin;
    let mut sty_given = args.style.is_some();
    let mut ind_given = args.output.is_some();
    let mut ilg_given = args.transcript.is_some();
    unsafe {
        init_page = args._init_page.is_some() as i32;
    }

    let mut log_given = match args._init_page {
        Some(InitialPage::Even | InitialPage::Odd | InitialPage::Any) => true,
        _ => false,
    };
    unsafe {
        german_sort = args._german_sort as i32;
    }

    if let Some(style) = args.style {
        unsafe { open_sty(CString::new(style).unwrap().into_raw()) }
    }

    if let Some(out) = args.output {
        unsafe { ind_fn = CString::new(out).unwrap().into_raw() }
    }

    if let Some(trans) = args.transcript {
        unsafe { ilg_fn = CString::new(trans).unwrap().into_raw() }
    }

    if let Some(page) = args._init_page {
        unsafe {
            pageno.copy_from_slice(std::mem::transmute::<&[u8], &[i8]>(
                CString::new(page.to_string()).unwrap().to_bytes(),
            ))
        }
    } else {
        unsafe { init_page = 0; }
    }

    unsafe {
        pgm_fn = CString::new(
            std::env::args()
                .into_iter()
                .next()
                .unwrap()
                .split('/')
                .last()
                .unwrap(),
        )
        .unwrap()
        .into_raw();
    }

    if args.input_files.len() >= 1024 {
        eprintln!("Too many input files (max {}).", 1024);
        unsafe { exit(1) }
    }

    for f in args.input_files.clone() {
        // TODO: Use proper types and stop unwrapping everything
        unsafe {
            check_idx(CString::new(f.as_str()).unwrap().as_ptr() as *mut i8, 0);
            // 0 = FALSE
        }
    }
    if !args.use_stdin && args.input_files.len() == 0 {
        // TODO: use stdin if no input file
    }

    if args.input_files.len() == 1 && !sty_given {
        let mut tmp = [0; 261];
        unsafe {
            sprintf(
                tmp.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                base.as_mut_ptr(),
                b".mst\0" as *const u8 as *const libc::c_char,
            );
        }
        // If we have read access to the file
        // TODO: use std::fs function
        if 0 == unsafe { access(tmp.as_mut_ptr(), 4) } {
            // TODO: Use proper types and stop unwrapping and using unsafe
            args.style = Some(
                unsafe { CString::from_raw(base.as_mut_ptr()) }
                    .into_string()
                    .unwrap()
                    + ".mst",
            );
            unsafe {
                open_sty(tmp.as_mut_ptr());
            }
        }
    }
    // TODO: temp waiting to use proper types
    // let mut cstring_inputs = args.input_files.clone().iter().map(|arg| CString::new(arg.as_str()).unwrap()).map(|s| s.as_ptr() as *mut i8).collect::<Vec<_>>();
    let cstring_inputs = args
        .input_files
        .clone()
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap().into_raw())
        .collect::<Vec<_>>();
    unsafe {
        fn_no = (cstring_inputs.len() - 1) as i32;
    }
    fns[..cstring_inputs.len()].copy_from_slice(cstring_inputs.as_slice());
    unsafe {
        process_idx(
            fns.as_mut_ptr(),
            use_stdin,
            sty_given,
            ind_given,
            ilg_given,
            log_given,
        );
    }

    unsafe {
        idx_gt = idx_tt - idx_et;
    }
    if args.input_files.len() > 1 {
        if unsafe { verbose } {
            eprintln!(
                "Overall {} files read ({} entries accepted, {} rejected).",
                unsafe { fn_no } + 1,
                unsafe { idx_gt },
                unsafe { idx_et }
            );
        }

        unsafe {
            fprintf(
                ilg_fp,
                b"Overall %d files read (%d entries accepted, %d rejected).\n\0" as *const u8
                    as *const libc::c_char,
                fn_no + 1,
                idx_gt,
                idx_et,
            );
        }
    }
    if unsafe { idx_gt } > 0 {
        unsafe {
            prepare_idx();
            sort_idx();
            gen_ind();
        }
        if unsafe { verbose } {
            // unsafe { exit(1); }
            eprintln!(
                "Output written in {}.",
                unsafe { CStr::from_ptr(ind_fn) }.to_str().unwrap()
            );
        }
        unsafe {
            fprintf(
                ilg_fp,
                b"Output written in %s.\n\0" as *const u8 as *const libc::c_char,
                ind_fn,
            );
        }
    } else {
        if unsafe { verbose } {
            eprintln!(
                "Nothing written in {}.",
                unsafe { CStr::from_ptr(ind_fn) }.to_str().unwrap()
            );
        }
        unsafe {
            fprintf(
                ilg_fp,
                b"Nothing written in %s.\n\0" as *const u8 as *const libc::c_char,
                ind_fn,
            );
        }
    }
    if unsafe { verbose } {
        eprintln!(
            "Transcript written in {}.",
            unsafe { CStr::from_ptr(ilg_fn) }.to_str().unwrap()
        );
    }
    unsafe {
        fprintf(
            ilg_fp,
            b"Transcript written in %s.\n\0" as *const u8 as *const libc::c_char,
            ilg_fn,
        );
    }
    unsafe {
        fclose(ind_fp);
        fclose(ilg_fp);
    }

    unsafe {
        exit(0);
    }
}
unsafe extern "C" fn prepare_idx() {
    let mut ptr = head;
    let mut i = 0;
    if head.is_null() {
        eprintln!("No valid index entries collected.");
        exit(1);
    }
    idx_key = calloc(
        idx_gt.try_into().unwrap(),
        ::core::mem::size_of::<FIELD_PTR>(),
    ) as *mut FIELD_PTR;
    if idx_key.is_null() {
        eprintln!("Not enough core...abort.");
        exit(1);
    }
    i = 0;
    while i < idx_gt {
        let fresh0 = &mut (*idx_key.offset(i as isize));
        *fresh0 = &mut (*ptr).data;
        ptr = (*ptr).next;
        i += 1;
    }
}
unsafe extern "C" fn process_idx(
    mut fn_0: *mut *mut libc::c_char,
    mut use_stdin: bool,
    mut sty_given: bool,
    mut ind_given: bool,
    mut ilg_given: bool,
    mut log_given: bool,
) {
    let mut i = 0;
    if fn_no == -(1) {
        use_stdin = true;
    } else {
        check_all(*fn_0.offset(0), ind_given, ilg_given, log_given);
        if unsafe { verbose } {
            eprintln!(
                "This is {}, {}",
                CStr::from_ptr(pgm_fn).to_str().unwrap(),
                "portable version 2.12 [26-May-1993]"
            );
        }
        fprintf(
            ilg_fp,
            b"This is %s, %s.\n\0" as *const u8 as *const libc::c_char,
            pgm_fn,
            b"portable version 2.12 [26-May-1993]\0" as *const u8 as *const libc::c_char,
        );
        need_version = false;
        if sty_given {
            scan_sty();
        }
        if german_sort != 0 && idx_quote as i32 == '"' as i32 {
            eprintln!(
                "Option -g invalid, quote character must be different from '{}'.",
                '"'
            );
            exit(1);
        }
        scan_idx();
        ind_given = true;
        ilg_given = true;
        i = 1;
        while i <= fn_no {
            check_idx(*fn_0.offset(i as isize), 1);
            scan_idx();
            i += 1;
        }
    }
    if use_stdin {
        idx_fn = b"stdin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        idx_fp = stdin();
        if ind_given {
            if ind_fp.is_null() && {
                ind_fp = fopen(ind_fn, b"w\0" as *const u8 as *const libc::c_char);
                ind_fp.is_null()
            } {
                eprintln!(
                    "Can't create output index file {}.",
                    CStr::from_ptr(ind_fn).to_str().unwrap()
                );
                exit(1);
            }
        } else {
            ind_fn = b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ind_fp = stdout();
        }
        if ilg_given {
            if ilg_fp.is_null() && {
                ilg_fp = fopen(ilg_fn, b"w\0" as *const u8 as *const libc::c_char);
                ilg_fp.is_null()
            } {
                eprintln!(
                    "Can't create transcript file {}.",
                    CStr::from_ptr(ilg_fn).to_str().unwrap()
                );
                exit(1);
            }
        } else {
            ilg_fn = b"stderr\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ilg_fp = stderr();
        }
        if fn_no == -(1) && sty_given {
            scan_sty();
        }
        if german_sort != 0 && idx_quote as i32 == '"' as i32 {
            eprintln!(
                "Option -g ignored, quote character must be different from '{}'.\n\0",
                '"'
            );
            exit(1);
        }
        if need_version {
            if unsafe { verbose } {
                eprintln!(
                    "This is {}, {}",
                    CStr::from_ptr(pgm_fn).to_str().unwrap(),
                    "portable version 2.12 [26-May-1993]"
                );
            }
            fprintf(
                ilg_fp,
                b"This is %s, %s.\n\0" as *const u8 as *const libc::c_char,
                pgm_fn,
                b"portable version 2.12 [26-May-1993]\0" as *const u8 as *const libc::c_char,
            );
            need_version = false;
        }
        scan_idx();
        fn_no += 1;
    }
}
unsafe extern "C" fn check_idx(mut fn_0: *mut libc::c_char, mut open_fn: i32) {
    let mut ptr = fn_0;
    let mut ext = std::ptr::null_mut::<libc::c_char>();
    let mut with_ext = 0;
    let mut i = 0;
    ext = strrchr(fn_0, '.' as i32);
    if !ext.is_null() && ext != fn_0 && *ext.offset(1) as i32 != '/' as i32 {
        with_ext = 1;
        while ptr != ext && i < 256 {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            let fresh2 = i;
            i += 1;
            base[fresh2 as usize] = *fresh1;
        }
    } else {
        while *ptr as i32 != '\0' as i32 && i < 256 {
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            let fresh4 = i;
            i += 1;
            base[fresh4 as usize] = *fresh3;
        }
    }
    if i < 256 {
        base[i as usize] = '\0' as i32 as libc::c_char;
    } else {
        eprintln!(
            "Index file name {} too long (max {}).",
            CStr::from_ptr(base.as_ptr()).to_str().unwrap(),
            256,
        );
        exit(1);
    }
    idx_fn = fn_0;
    if open_fn != 0 && {
        idx_fp = fopen(idx_fn, b"r\0" as *const u8 as *const libc::c_char);
        idx_fp.is_null()
    } || open_fn == 0 && access(idx_fn, 4) != 0
    {
        if with_ext != 0 {
            eprintln!(
                "Input index file {} not found.",
                CStr::from_ptr(idx_fn).to_str().unwrap(),
            );
            exit(1);
        } else {
            idx_fn = malloc(256) as *mut libc::c_char;
            if idx_fn.is_null() {
                eprintln!("Not enough core...abort.");
                exit(1);
            }
            sprintf(
                idx_fn,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                base.as_mut_ptr(),
                b".idx\0" as *const u8 as *const libc::c_char,
            );
            if open_fn != 0 && {
                idx_fp = fopen(idx_fn, b"r\0" as *const u8 as *const libc::c_char);
                idx_fp.is_null()
            } || open_fn == 0 && access(idx_fn, 4) != 0
            {
                eprintln!(
                    "Couldn't find input index file {} nor {}.",
                    CStr::from_ptr(base.as_ptr()).to_str().unwrap(),
                    CStr::from_ptr(idx_fn).to_str().unwrap(),
                );
                exit(1);
            }
        }
    }
}
unsafe extern "C" fn check_all(
    mut fn_0: *mut libc::c_char,
    mut ind_given: bool,
    mut ilg_given: bool,
    mut log_given: bool,
) {
    check_idx(fn_0, 1);
    if !ind_given {
        sprintf(
            ind.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".ind\0" as *const u8 as *const libc::c_char,
        );
        ind_fn = ind.as_mut_ptr();
    }
    ind_fp = fopen(ind_fn, b"w\0" as *const u8 as *const libc::c_char);
    if ind_fp.is_null() {
        eprintln!(
            "Can't create output index file {}.",
            CStr::from_ptr(ind_fn).to_str().unwrap(),
        );
        exit(1);
    }
    if !ilg_given {
        sprintf(
            ilg.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".ilg\0" as *const u8 as *const libc::c_char,
        );
        ilg_fn = ilg.as_mut_ptr();
    }
    ilg_fp = fopen(ilg_fn, b"w\0" as *const u8 as *const libc::c_char);
    if ilg_fp.is_null() {
        eprintln!(
            "Can't create transcript file {}.",
            CStr::from_ptr(ilg_fn).to_str().unwrap(),
        );
        exit(1);
    }
    if log_given {
        sprintf(
            log_fn.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".log\0" as *const u8 as *const libc::c_char,
        );
        log_fp = fopen(
            log_fn.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if log_fp.is_null() {
            eprintln!(
                "Source log file {} not found.",
                CStr::from_ptr(log_fn.as_ptr()).to_str().unwrap(),
            );
            exit(1);
        } else {
            find_pageno();
            fclose(log_fp);
        }
    }
}
unsafe extern "C" fn find_pageno() {
    let mut i = 0;
    let mut p = 0;
    let mut c = 0;
    fseek(log_fp, -(1), 2);
    p = fgetc(log_fp);
    fseek(log_fp, -(2), 1);
    loop {
        c = p;
        p = fgetc(log_fp);
        if p == '[' as i32
            && *(*__ctype_b_loc()).offset(c as isize) as i32
                & _ISdigit as i32 as libc::c_ushort as i32
                != 0
            || fseek(log_fp, -(2), 1) != 0
        {
            break;
        }
    }
    if p == '[' as i32 {
        loop {
            c = fgetc(log_fp);
            if c != ' ' as i32 {
                break;
            }
        }
        loop {
            let fresh5 = i;
            i += 1;
            pageno[fresh5 as usize] = c as libc::c_char;
            c = fgetc(log_fp);
            if *(*__ctype_b_loc()).offset(c as isize) as i32
                & _ISdigit as i32 as libc::c_ushort as i32
                == 0
            {
                break;
            }
        }
        pageno[i as usize] = '\0' as i32 as libc::c_char;
    } else {
        fprintf(
            ilg_fp,
            b"Couldn't find any page number in %s...ignored\n\0" as *const u8
                as *const libc::c_char,
            log_fn.as_mut_ptr(),
        );
        init_page = 0;
    };
}
unsafe extern "C" fn open_sty(mut fn_0: *mut libc::c_char) {
    let mut path = std::ptr::null_mut::<libc::c_char>();
    let mut ptr = std::ptr::null_mut::<libc::c_char>();
    let mut i = 0;
    let mut len = 0;
    path = getenv(b"INDEXSTYLE\0" as *const u8 as *const libc::c_char);
    if path.is_null() {
        strcpy(sty_fn.as_mut_ptr(), fn_0);
        sty_fp = fopen(
            sty_fn.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
    } else {
        len = (1024usize).wrapping_sub(strlen(fn_0)).wrapping_sub(1) as i32;
        while *path as i32 != '\0' as i32 {
            ptr = strchr(path, ':' as i32);
            i = 0;
            if ptr.is_null() {
                let mut j = strlen(path) as i32;
                while i < j {
                    let fresh6 = path;
                    path = path.offset(1);
                    let fresh7 = i;
                    i += 1;
                    sty_fn[fresh7 as usize] = *fresh6;
                }
            } else {
                while path != ptr && i < len {
                    let fresh8 = path;
                    path = path.offset(1);
                    let fresh9 = i;
                    i += 1;
                    sty_fn[fresh9 as usize] = *fresh8;
                }
            }
            if i == len {
                eprintln!(
                    "Path {} too long (max {}).",
                    CStr::from_ptr(sty_fn.as_ptr()).to_str().unwrap(),
                    1024,
                );
                exit(1);
            } else {
                if sty_fn[(i - 1) as usize] as i32 != ']' as i32 {
                    let fresh10 = i;
                    i += 1;
                    sty_fn[fresh10 as usize] = '/' as i32 as libc::c_char;
                }
                sty_fn[i as usize] = '\0' as i32 as libc::c_char;
                strcat(sty_fn.as_mut_ptr(), fn_0);
                sty_fp = fopen(
                    sty_fn.as_mut_ptr(),
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if !sty_fp.is_null() {
                    break;
                }
                path = path.offset(1);
            }
        }
    }
    if sty_fp.is_null() {
        eprintln!(
            "Index style file {} not found.",
            CStr::from_ptr(fn_0).to_str().unwrap(),
        );
        exit(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn strtoint(mut str: *mut libc::c_char) -> i32 {
    let mut val = 0;
    while *str as i32 != '\0' as i32 {
        val = 10 * val + *str as i32 - 48;
        str = str.offset(1);
    }
    val
}
