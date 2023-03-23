use quote::quote;
use std::{path::{Path, PathBuf}, fs::File, io::Read};
use syn::{DeriveInput};
use walkdir::{DirEntry, WalkDir};

//docs > https://docs.rs/syn/latest/syn/
//or 
//https://tree-sitter.github.io/tree-sitter/ (used even by sourcegraph so it's pretty good for rust or general purpose)

//Other tooling (Git diff)
//https://github.com/dandavison/delta , https://dandavison.github.io/delta/usage.html

/*Example 
 * let source_code = r#"
    struct MyStruct1 {
        // fields...
    }

    #[derive(SomeTrait)]
    struct MyStruct2 {
        // fields...
    }

    struct MyStruct3 {
        // fields...
    }
"#;

 */

fn visit_file(entry: &DirEntry) {
   let path = "./input.rs";
   
   let mut file = File::open(path).expect("Failed to open file");
   let mut contents = Vec::new();
   file.read_to_end(&mut contents)
       .expect("Failed to read file");

   todo!();
   //  if let Ok(file_type) = entry.file_type() {
   //      if file_type.is_file() && entry.file_name().to_string_lossy().ends_with(".rs") {
   //          let source_code = std::fs::read_to_string(entry.path()).expect("Failed to read file");
   //          let ast = parse_file(&source_code).expect("Failed to parse file");

   //          // Iterate through all items in the AST
   //          for item in ast.items.iter() {
   //             // Check if the item is a struct
   //              if let Item::Struct(item_struct) = item {
   //                  if item_struct.attrs.iter().any(|attr| {
   //                      if let Ok(DeriveInput { path, .. }) = syn::parse2(attr.tokens.clone()) {
   //                          path.is_ident("derive")
   //                              && path
   //                                  .segments
   //                                  .iter()
   //                                  .any(|segment| segment.ident == "SomeTrait")
   //                      } else {
   //                          false
   //                      }
   //                  }) {
   //                      let struct_ident = &item_struct.ident;
   //                      let generated_code = quote! {
   //                          println!("Found struct {} that implements SomeTrait in file {}!", stringify!(#struct_ident), entry.path().display());
   //                      };
   //                      println!("{}", generated_code);
   //                  }
   //              }
   //          }
   //      }
   //  }
}

fn walk_directory<P: AsRef<Path>>(path: P) {
    let mut entries = WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| !entry.file_name().to_string_lossy().starts_with("."))
        .filter_map(|entry| entry.ok());

    while let Some(entry) = entries.next() {
        visit_file(&entry);
    }
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    walk_directory(current_dir);
}
