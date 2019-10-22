//! Extra stuff from the libvmaf git repo that you’ll probably need.
use std::path::PathBuf;
use lazy_static::lazy_static;
use tempfile::{NamedTempFile, TempDir};


///////////////////////////////////////////////////////////////////////////////
// CORE
///////////////////////////////////////////////////////////////////////////////

const buildtype_docs_only_stub_msg: &'static str =
    "stub for docs.rs only (when cargo feature = 'buildtype-docs-only')";

/// Returns the string contents of the VMAF 'default' model that this library
/// was compiled with. 
/// 
/// Regarding the return tuple. I have no idea what the first is, in
/// relation to the second. Yet the first component (in the tuple) is
/// what you give VMAF (dumped to a file).
/// 
/// The second is also for VMAF, but it looks for it with the same file
/// path as the first, except ending with a `.model` extension. There is
/// no documentation about this, yet it seems to be what it wants at
/// runtime. 
#[cfg(not(feature="buildtype-docs-only"))]
pub fn get_vmaf_def_model() -> (&'static str, &'static str) {
    let x = include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_v0.6.1.pkl"));
    let y = include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_v0.6.1.pkl.model"));
    (x, y)
}

/// Returns the string contents of the VMAF '4K' model that this library
/// was compiled with. 
/// 
/// Regarding the return tuple. I have no idea what the first is, in
/// relation to the second. Yet the first component (in the tuple) is
/// what you give VMAF (dumped to a file).
/// 
/// The second is also for VMAF, but it looks for it with the same file
/// path as the first, except ending with a `.model` extension. There is
/// no documentation about this, yet it seems to be what it wants at
/// runtime. 
#[cfg(not(feature="buildtype-docs-only"))]
pub fn get_vmaf_4k_model() -> (&'static str, &'static str) {
    let x = include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_4k_v0.6.1.pkl"));
    let y = include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_4k_v0.6.1.pkl.model"));
    (x, y)
}

/// Returns the string contents of the VMAF 'default' model that this library
/// was compiled with. 
/// 
/// Regarding the return tuple. I have no idea what the first is, in
/// relation to the second. Yet the first component (in the tuple) is
/// what you give VMAF (dumped to a file).
/// 
/// The second is also for VMAF, but it looks for it with the same file
/// path as the first, except ending with a `.model` extension. There is
/// no documentation about this, yet it seems to be what it wants at
/// runtime. 
#[cfg(feature="buildtype-docs-only")]
pub fn get_vmaf_def_model() -> (&'static str, &'static str) {panic!(buildtype_docs_only_stub_msg)}


/// Returns the string contents of the VMAF '4K' model that this library
/// was compiled with. 
/// 
/// Regarding the return tuple. I have no idea what the first is, in
/// relation to the second. Yet the first component (in the tuple) is
/// what you give VMAF (dumped to a file).
/// 
/// The second is also for VMAF, but it looks for it with the same file
/// path as the first, except ending with a `.model` extension. There is
/// no documentation about this, yet it seems to be what it wants at
/// runtime. 
#[cfg(feature="buildtype-docs-only")]
pub fn get_vmaf_4k_model() -> (&'static str, &'static str) {panic!(buildtype_docs_only_stub_msg)}


///////////////////////////////////////////////////////////////////////////////
// HIGHER LEVEL UTILS - FILE-SYSTEM
///////////////////////////////////////////////////////////////////////////////

/// Internal - created by the lazy static macro.
struct TmpVmafModelFile {
    /// Keep this in memory, when it gets ‘Dropped’, it’ll remove the tmp
    /// directory.
    dir: TempDir,
    path: PathBuf,
}

lazy_static! {
    static ref TMP_VMAF_DEF_MODEL_FILE: TmpVmafModelFile = {
        // INIT DIR
        let mut root_dir = TempDir::new().expect("TempDir::new() failed");
        // FILE PATHS
        let mut model_pkg = PathBuf::from(root_dir.path())
            .join("vmaf_v0.6.1.pkl");
        let mut model_other = PathBuf::from(root_dir.path())
            .join("vmaf_v0.6.1.pkl.model");
        // FILL FILES
        let (pkg, other) = get_vmaf_4k_model();
        std::fs::write(&model_pkg, pkg).expect("TmpVmafModelFile::new_tmp");
        std::fs::write(&model_other, other).expect("TmpVmafModelFile::new_tmp");
        // DONE
        TmpVmafModelFile {
            dir: root_dir,
            path: model_pkg,
        }
    };

    static ref TMP_VMAF_4K_MODEL_FILE: TmpVmafModelFile = {
        // INIT DIR
        let mut root_dir = TempDir::new().expect("TempDir::new() failed");
        // FILE PATHS
        let mut model_pkg = PathBuf::from(root_dir.path())
            .join("vmaf_4k_v0.6.1.pkl");
        let mut model_other = PathBuf::from(root_dir.path())
            .join("vmaf_4k_v0.6.1.pkl.model");
        // FILL FILES
        let (pkg, other) = get_vmaf_4k_model();
        std::fs::write(&model_pkg, pkg).expect("TmpVmafModelFile::new_tmp");
        std::fs::write(&model_other, other).expect("TmpVmafModelFile::new_tmp");
        // DONE
        TmpVmafModelFile {
            dir: root_dir,
            path: model_pkg,
        }
    };
}


/// The VMAF default model.
/// 
/// This internally creates a temporary file that lasts for the duration of th
/// process. The temporary file is automatically removed at the end of the
/// process (or presumably by the OS following a reboot).
///
/// This is a relatively cheap operation in the sense that this only happens
/// once. All subsequent calls point to the original file (i.e. this is an
/// idempotent operation).
pub fn get_def_model_path() -> PathBuf {
    TMP_VMAF_DEF_MODEL_FILE.path.clone()
}


/// The VMAF 4K model.
/// 
/// This internally creates a temporary file that lasts for the duration of th
/// process. The temporary file is automatically removed at the end of the
/// process (or presumably by the OS following a reboot).
///
/// This is a relatively cheap operation in the sense that this only happens
/// once. All subsequent calls point to the original file (i.e. this is an
/// idempotent operation). 
pub fn get_4k_model_path() -> PathBuf {
    TMP_VMAF_4K_MODEL_FILE.path.clone()
}