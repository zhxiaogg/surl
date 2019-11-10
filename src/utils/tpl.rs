use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
    RenderError,
};
use serde_json;
use std::io::Write;

pub fn to_json(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    let s = serde_json::to_string(param.value()).unwrap();
    out.write(s.as_ref())?;
    Ok(())
}
