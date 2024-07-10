use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/gen.h")
        .use_core()
        .derive_default(true)
        .generate_cstr(true)
        .clang_args(&[
            "--target=x86_64-pc-windows-gnu",
            "-fms-extensions",
            "-DHRESULT=long",
            "-I./headers/Win11_2309_23H2/x64/System32/ntoskrnl.exe/"
        ])
        .must_use_type("HRESULT")
        .must_use_type("NTSTATUS")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
