use assembly::{ast::ModuleAst, Library, LibraryNamespace, MaslLibrary, Version};
use std::{collections::BTreeMap, env, fs, io, path::Path};

mod md_renderer;
use md_renderer::MarkdownRenderer;

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "./asm";
const ASL_DIR_PATH: &str = "./assets";
const DOC_DIR_PATH: &str = "./docs";

// TYPE ALIASES and HELPER STRUCTS
// ================================================================================================

type ModuleMap = BTreeMap<String, ModuleAst>;

// PRE-PROCESSING
// ================================================================================================

/// Read and parse the contents from `./asm` into a `LibraryContents` struct, serializing it into
/// `assets` folder under `std` namespace.
fn main() -> io::Result<()> {
    // re-build the `[OUT_DIR]/assets/std.masl` file iff something in the `./asm` directory
    // or its builder changed:
    println!("cargo:rerun-if-changed=asm");
    println!("cargo:rerun-if-changed=../assembly/src");

    let namespace = LibraryNamespace::try_from("std".to_string()).expect("invalid base namespace");
    let version = Version::try_from(env!("CARGO_PKG_VERSION")).expect("invalid cargo version");
    let locations = true; // store & load locations by default
    let stdlib = MaslLibrary::read_from_dir(ASM_DIR_PATH, namespace, locations, version)?;
    let docs = stdlib
        .modules()
        .map(|module| (module.path.to_string(), module.ast.clone()))
        .collect();

    // write the masl output
    let build_dir = env::var("OUT_DIR").unwrap();
    stdlib.write_to_dir(Path::new(&build_dir).join(ASL_DIR_PATH))?;

    // updates the documentation of these modules
    build_stdlib_docs(&docs, DOC_DIR_PATH)?;

    Ok(())
}

// STDLIB DOCUMENTATION
// ================================================================================================

/// A renderer renders a ModuleSourceMap into a particular doc format and index (e.g: markdown, etc)
trait Renderer {
    // Render writes out the document files into the output directory
    fn render(stdlib: &ModuleMap, output_dir: &str);
}

/// Writes Miden standard library modules documentation markdown files based on the available modules and comments.
pub fn build_stdlib_docs(module_map: &ModuleMap, output_dir: &str) -> io::Result<()> {
    // Clean the output folder. This only deletes the folder's content, and not the folder itself,
    // because removing the folder fails on docs.rs
    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            assert!(metadata.is_file());
            fs::remove_file(entry.path())?;
        }
    }

    // Render the stdlib struct into markdown
    // TODO: Make the renderer choice pluggable.
    MarkdownRenderer::render(module_map, output_dir);

    Ok(())
}
