use anyhow::{Context, Result};

mod code_gen;
mod parsing;

use parsing::*;

fn main() -> Result<()> {
    // Obtain all modules
    let modules = get_modules(&std::path::PathBuf::from_iter(["WrapVTK", "build", "xml"]))?;

    use rayon::prelude::*;
    modules
        .into_par_iter()
        .map(|module| {
            for file in module.files.iter() {
                for class in file.1.classes.iter() {
                    let wrapper = gen_wrapper(class)?;
                    println!("{wrapper}");
                }
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}
