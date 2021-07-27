mod array;
mod error;
mod string;

use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let array = array::init_module(&mut cx)?;
    cx.export_value("array", array)?;

    let error = error::init_module(&mut cx)?;
    cx.export_value("error", error)?;

    let string = string::init_module(&mut cx)?;
    cx.export_value("string", string)?;

    Ok(())
}
