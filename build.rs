#[allow(dead_code)]
use std::env;
use std::fs;
use std::path::PathBuf;
use tempdir::TempDir;
use std::error::Error;

struct Target {
    pub generated: PathBuf,
    pub include: String,
    pub is_cpp: bool
}

impl Target {
    pub fn new(generated: PathBuf, include: String, is_cpp: bool) -> Self {
        Self {
            generated,
            include,
            is_cpp
        }
    }
}

fn fix_c_header(data: &str) -> String {
    let mut fixed = String::new();

    let mut ignore_block = false;
    for line in data.lines() {
        if ignore_block {
            if line.starts_with('}') {
                ignore_block = false;
            }

            continue;
        }

        if line.starts_with("class") || line.contains("::") || line.contains(|c| "<>".contains(c)) || line.contains("TAG_UNNAMED") {
            if !line.ends_with(';') {
                ignore_block = true;
            }

            continue;
        }

        fixed.push_str(line);
        fixed.push('\n');
    }

    fixed
}

fn fix_cpp_header(data: &str) -> String {
    let mut fixed = String::new();

    for line in data.lines() {
        if !line.starts_with("enum") || !line.ends_with(';') {
            fixed.push_str(line);
            fixed.push('\n');
            continue;
        }

        fixed.push_str(&line.replace(";", " : int;"));
        fixed.push('\n');
    }

    fixed
}

#[cfg(feature = "build")]
fn main() -> Result<(), Box<dyn Error>> {
    let mut out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    out_dir.push("src");

    #[allow(unused_mut)]
    let mut targets: Vec<Target> = vec![];

    #[cfg(feature = "hal")]
    targets.push(Target::new(out_dir.join("hal.rs"), String::from("hal.dll"), false));

    #[cfg(feature = "ntdll")]
    targets.push(Target::new(out_dir.join("ntdll.rs"), String::from("ntdll.dll"), false));

    #[cfg(feature = "ntoskrnl")]
    targets.push(Target::new(out_dir.join("ntoskrnl.rs"), String::from("ntoskrnl.exe"), false));

    #[cfg(feature = "ole32")]
    targets.push(Target::new(out_dir.join("ole32.rs"), String::from("ole32.dll"), true));


    let include_dir = TempDir::new("ntdiff_rs")?;
    let fixed_header = include_dir.path().join("ALL.h");
    for target in targets {
        let header = fs::read_to_string(format!("./ntdiff/Win11_2309_23H2/x64/System32/{}/ALL.h", target.include))?;


        let fixed = target.is_cpp.then(|| fix_cpp_header(&header)).unwrap_or_else(|| header);
        fs::write(&fixed_header, &fixed)?;
        fs::write("/tmp/fixed.h", &fixed)?;

        let mut builder = bindgen::Builder::default()
            .header("src/gen.h")
            .derive_default(true)
            .generate_cstr(true)
            .clang_args(&[
                "--target=x86_64-pc-windows-gnu",
                "-fms-extensions",
                "-DHRESULT=long",
            ])
            .clang_arg(format!("-I{}", include_dir.path().display()))
            .must_use_type("HRESULT")
            .must_use_type("NTSTATUS");

        if target.is_cpp {
            builder = builder.clang_arg("-xc++");
        }

        #[cfg(not(feature = "std"))]
        let builder = builder.use_core();

        let bindings = builder.generate()?;

        bindings
            .write_to_file(target.generated)?
    }

    Ok(())
}


#[cfg(not(feature = "build"))]
fn main() {}