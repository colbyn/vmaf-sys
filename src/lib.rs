#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


///////////////////////////////////////////////////////////////////////////////
// EXTRAS (FROM LIBVMAF SOURCE CODE)
///////////////////////////////////////////////////////////////////////////////

/// Extra stuff from the libvmaf git repo that you’ll probably need.
pub mod extras {
    const buildtype_docs_only_stub_msg: &'static str =
        "stub for docs.rs only (when cargo feature = 'buildtype-docs-only')";

    /// Returns the string contents of the VMAF 'default' model that this library
    /// was compiled with. 
    #[cfg(not(feature="buildtype-docs-only"))]
    pub fn get_vmaf_def_model() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_v0.6.1.pkl"))
    }

    /// Returns the string contents of the VMAF '4K' model that this library
    /// was compiled with. 
    #[cfg(not(feature="buildtype-docs-only"))]
    pub fn get_vmaf_4k_model() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_4k_v0.6.1.pkl"))
    }

    /// Returns the string contents of the VMAF 'default' model that this library
    /// was compiled with. 
    #[cfg(feature="buildtype-docs-only")]
    pub fn get_vmaf_def_model() -> &'static str {panic!(buildtype_docs_only_stub_msg)}


    /// Returns the string contents of the VMAF '4K' model that this library
    /// was compiled with. 
    #[cfg(feature="buildtype-docs-only")]
    pub fn get_vmaf_4k_model() -> &'static str {panic!(buildtype_docs_only_stub_msg)}
}
