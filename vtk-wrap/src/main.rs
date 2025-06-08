use anyhow::{Context, Result};

mod code_gen;
mod inheritance_hierarchy;
mod parsing;

use code_gen::*;
use inheritance_hierarchy::*;
use parsing::*;

fn main() -> Result<()> {
    // Obtain all modules
    let modules = get_modules("WrapVTK/build/xml/*ChartsCore")?;

    let file = &modules[0].files[0].1;
    for class in file.classes.iter() {
        let wrapper = gen_wrapper(class)?;
        println!("{wrapper}");
    }

    Ok(())
}
