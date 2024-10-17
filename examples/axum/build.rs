use static_file_util::process_file;

fn main() {
    process_file("images/crab.svg", "crab_svg_HASH");
    process_file("css/styles.css", "styles_css_HASH");
}
