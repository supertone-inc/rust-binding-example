use neon::prelude::*;

pub fn init_module<'a, C: Context<'a>>(c: &mut C) -> JsResult<'a, JsObject> {
    let exports = JsObject::new(c);

    let concat = JsFunction::new(c, concat)?;
    exports.set(c, "concat", concat)?;

    Ok(exports)
}

fn concat(mut cx: FunctionContext) -> JsResult<JsArray> {
    let a: Vec<f32> = cx
        .argument::<JsArray>(0)?
        .to_vec(&mut cx)?
        .iter()
        .map(|js_value| js_value.downcast_or_throw(&mut cx))
        .collect::<NeonResult<Vec<Handle<JsNumber>>>>()?
        .iter()
        .map(|js_number| js_number.value(&mut cx) as f32)
        .collect();

    let b: Vec<f32> = cx
        .argument::<JsArray>(1)?
        .to_vec(&mut cx)?
        .iter()
        .map(|js_value| js_value.downcast_or_throw(&mut cx))
        .collect::<NeonResult<Vec<Handle<JsNumber>>>>()?
        .iter()
        .map(|js_number| js_number.value(&mut cx) as f32)
        .collect();

    let c = hello::array::concat(&a, &b);

    let js_array = JsArray::new(&mut cx, c.len() as u32);
    c.iter().enumerate().try_for_each(|(i, value)| -> NeonResult<()> {
        let js_number = cx.number(*value);
        js_array.set(&mut cx, i as u32, js_number)?;
        Ok(())
    })?;

    Ok(js_array)
}
