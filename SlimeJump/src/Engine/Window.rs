use sfml;

struct Window {
    width: u32, height: u32,
    title: &'static str,
    style: sfml::WindowStyle
}