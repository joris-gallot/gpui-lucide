//! Build script to generate IconName enum and paths from SVG files

use heck::ToUpperCamelCase;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let icons_dir = Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("icons");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icons_generated.rs");

    println!("cargo:rerun-if-changed={}", icons_dir.display());

    let mut icon_entries: Vec<(String, String, String)> = Vec::new();

    if icons_dir.exists() {
        let mut entries: Vec<_> = fs::read_dir(&icons_dir)
            .expect("Failed to read icons directory")
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "svg")
                    .unwrap_or(false)
            })
            .collect();

        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            let variant_name = file_stem.to_upper_camel_case();

            // Handle names starting with numbers
            let variant_name = if variant_name
                .chars()
                .next()
                .map(|c| c.is_numeric())
                .unwrap_or(false)
            {
                format!("N{}", variant_name)
            } else {
                variant_name
            };

            let file_name = format!("{}.svg", file_stem);

            icon_entries.push((variant_name, file_stem.to_string(), file_name));
        }
    }

    let mut code = String::new();

    // Generate enum variants
    code.push_str("/// All available Lucide icon names.\n");
    code.push_str("///\n");
    code.push_str(
        "/// This enum is auto-generated from the SVG files in the `icons/` directory.\n",
    );
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    code.push_str("pub enum IconName {\n");

    for (variant_name, file_stem, _) in &icon_entries {
        code.push_str(&format!("    /// {}\n", file_stem));
        code.push_str(&format!("    {},\n", variant_name));
    }

    code.push_str("}\n\n");

    // Generate path() implementation
    code.push_str("impl IconName {\n");
    code.push_str("    /// Returns the asset path for this icon.\n");
    code.push_str("    pub fn path(&self) -> &'static str {\n");
    code.push_str("        match self {\n");

    for (variant_name, _, file_name) in &icon_entries {
        code.push_str(&format!(
            "            IconName::{} => \"icons/{}\",\n",
            variant_name, file_name
        ));
    }

    code.push_str("        }\n");
    code.push_str("    }\n\n");

    // Generate name() for display
    code.push_str("    /// Returns the display name (kebab-case) for this icon.\n");
    code.push_str("    pub fn name(&self) -> &'static str {\n");
    code.push_str("        match self {\n");

    for (variant_name, file_stem, _) in &icon_entries {
        code.push_str(&format!(
            "            IconName::{} => \"{}\",\n",
            variant_name, file_stem
        ));
    }

    code.push_str("        }\n");
    code.push_str("    }\n\n");

    // Generate all() iterator
    code.push_str("    /// Returns an iterator over all icon names.\n");
    code.push_str("    pub fn all() -> impl Iterator<Item = IconName> {\n");
    code.push_str("        [\n");

    for (variant_name, _, _) in &icon_entries {
        code.push_str(&format!("            IconName::{},\n", variant_name));
    }

    code.push_str("        ].into_iter()\n");
    code.push_str("    }\n\n");

    // Generate count
    code.push_str(&format!(
        "    /// Returns the total number of available icons ({}).\n",
        icon_entries.len()
    ));
    code.push_str("    pub const fn count() -> usize {\n");
    code.push_str(&format!("        {}\n", icon_entries.len()));
    code.push_str("    }\n");

    code.push_str("}\n\n");

    // Implement Display
    code.push_str("impl std::fmt::Display for IconName {\n");
    code.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    code.push_str("        write!(f, \"{}\", self.name())\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    fs::write(&dest_path, code).expect("Failed to write generated code");
}
