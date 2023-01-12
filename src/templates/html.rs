use handlebars::{Context, Handlebars};
use std::collections::HashMap;

pub fn render_template(handlebars: &Handlebars, template_name: &str, data: &HashMap<&str, &str>) -> String {
    handlebars.render(template_name, &Context::wraps(data)).unwrap()
}
