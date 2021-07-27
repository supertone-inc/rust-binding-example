mod array;
mod string;

use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let array = array::init_module(&mut cx)?;
    cx.export_value("array", array)?;

    let string = string::init_module(&mut cx)?;
    cx.export_value("string", string)?;

    Ok(())
}
