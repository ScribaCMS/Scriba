use handlebars::Handlebars;
use std::path::Path;

pub fn load_templates(handlebars: &mut Handlebars) {
    handlebars.register_templates_directory(".html", Path::new("templates")).unwrap();
}
