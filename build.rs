#![allow(unused)]

use std::path::PathBuf;
use std::string::ToString;
use flate2::read::GzDecoder;
use tar::Archive;

fn is_release_mode() -> bool {
    let value = std::env::var("PROFILE")
        .expect("missing PROFILE")
        .to_lowercase();
    &value == "release"
}

fn is_debug_mode() -> bool {
    let value = std::env::var("PROFILE")
        .expect("missing PROFILE")
        .to_lowercase();
    &value == "debug"
}

fn get_vmaf_output_dir() -> PathBuf {
    let vmaf_out_dir = std::env::var("OUT_DIR").expect("missing OUT_DIR");
    let vmaf_out_dir = PathBuf::from(vmaf_out_dir).join("vmaf");
    std::fs::create_dir_all(&vmaf_out_dir).expect("unable to add vmaf dir under OUT_DIR");
    vmaf_out_dir
}

fn run_make(source_path: &PathBuf) {
    let result = std::process::Command::new("make")
        .arg("-C")
        .arg(source_path)
        .output()
        .expect(&format!("make -C {:?} failed", source_path));
}


struct VmafFiles {
    release_dir: PathBuf,
    lib_file: PathBuf,
    model_def: PathBuf,
    model_4k: PathBuf,
    header_file: PathBuf,
}

fn download_and_build_vmaf() -> Result<VmafFiles, String> {
    let out_dir = get_vmaf_output_dir();
    let download_dir = out_dir.join("download");
    let source_dir = out_dir.join("source");
    let release_dir = out_dir.join("release");
    // OUTPUT (RELEASE) FILES
    let lib_file = release_dir.join("libvmaf.a");
    let model_def = release_dir.join("vmaf_v0.6.1.pkl");
    let model_4k = release_dir.join("vmaf_4k_v0.6.1.pkl");
    let header_file = release_dir.join("libvmaf.h");
    // CHECKS
    if is_debug_mode() {
        // Let’s not re-download this every time someone (or their dev tools)
        // builds the project. Unless in release mode.
        let all_exists = lib_file.exists()
            && model_def.exists()
            && model_4k.exists()
            && header_file.exists();
        if all_exists {
            return Ok(VmafFiles {
                release_dir: release_dir.clone(),
                lib_file,
                model_def,
                model_4k,
                header_file,
            });
        }
    }
    // CLEAN
    std::fs::remove_dir_all(&out_dir).map_err(|x| x.to_string())?;
    // SETUP
    std::fs::create_dir_all(&download_dir).map_err(|x| x.to_string())?;
    // DOWNLOAD
    let url = "https://github.com/Netflix/vmaf/tarball/master";
    let tar_reply = reqwest::get(url).expect("unable to get vmaf tar file from github");
    let tar = GzDecoder::new(tar_reply);
    let mut archive = Archive::new(tar);
    // UNPACK ARCHIVE
    let tmp_source_dir: Option<PathBuf> = {
        archive
            .unpack(&download_dir)
            .map_err(|x| format!(
                "failed to unpack vmaf tar payload from github to {:?}: {:?}",
                download_dir,
                x
            ))?;
        let xs = std::fs::read_dir(&download_dir)
            .expect(&format!("unable to read dir {:?}", download_dir))
            .filter_map(Result::ok)
            .filter(|file| {
                file.file_type()
                    .map(|x| x.is_dir())
                    .unwrap_or(false)
            })
            .collect::<Vec<std::fs::DirEntry>>();
        match &xs[..] {
            [x] => Some(x.path()),
            _ => None,
        }
    };
    // MOVE TO STD SOURCE DIR
    let tmp_source_dir = tmp_source_dir.expect("unexpected tar output from github");
    std::fs::rename(&tmp_source_dir, &source_dir)
        .map_err(|x| format!(
            "unable to rename from {:?} to {:?}: {}",
            tmp_source_dir,
            source_dir,
            x,
        ))?;
    // COMPILE SOURCE
    run_make(&source_dir);
    // TO RELEASE DIR
    std::fs::create_dir_all(&release_dir).map_err(|x| x.to_string())?;
    let cpy = |src: PathBuf, dest: &PathBuf| {
        std::fs::copy(&src, dest).expect(&format!(
            "unable to cpy from {:?} to {:?}",
            src,
            dest,
        ));
    };
    cpy(source_dir.join("src/libvmaf/src/libvmaf.h"), &header_file);
    cpy(source_dir.join("src/libvmaf/libvmaf.a"), &lib_file);
    cpy(source_dir.join("model/vmaf_v0.6.1.pkl"), &model_def);
    cpy(source_dir.join("model/vmaf_4k_v0.6.1.pkl"), &model_4k);
    // CLEANUP
    std::fs::remove_dir_all(&download_dir).map_err(|x| x.to_string())?;
    std::fs::remove_dir_all(&source_dir).map_err(|x| x.to_string())?;
    // DONE
    Ok(VmafFiles{
        release_dir: release_dir.clone(),
        lib_file,
        model_def,
        model_4k,
        header_file,
    })
}

#[cfg(not(feature = "docs-only"))]
fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // LINK C++ (VMAF REQUIREMENT)
    let target  = std::env::var("TARGET").expect("missing TARGET");
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    // DOWNLOAD & BUILD VMAF
    let vmaf_files = match download_and_build_vmaf() {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    };
    
    // LINK TO STATIC LIB
    println!("cargo:rustc-link-search=native={}", {
        vmaf_files.release_dir
            .to_str()
            .expect("unable to get str")
    });
    println!("cargo:rustc-link-lib=static=vmaf");
    
    // BUILD RUST FFI CODE
    bindgen::Builder::default()
        .header({
            vmaf_files.header_file
                .to_str()
                .expect("vmaf_files.header_file.to_str() failed")
        })
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


#[cfg(feature = "docs-only")]
fn main() {
    // BUILD RUST FFI CODE
    bindgen::Builder::default()
        .header("include/libvmaf.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
