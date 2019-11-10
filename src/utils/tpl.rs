use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
    RenderError,
};
use serde_json;
use std::io::Write;
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};
pub fn to_json(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    let s = serde_json::to_string(param.value()).unwrap();
    out.write(s.as_ref())?;
    Ok(())
}

pub fn unix_timestamp(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let now_in_seconds = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    };
    out.write(now_in_seconds.to_string().as_ref())?;
    Ok(())
}
