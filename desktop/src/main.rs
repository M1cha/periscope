use bpaf::Bpaf;
use kiss3d::{light::Light, window::Window};
use nalgebra::{Translation2, UnitComplex};
use std::path::Path;

#[derive(Bpaf, Debug, Clone)]
struct CommandLine {
    #[bpaf(short('a'), long)]
    pub switch_addr: String,
    #[bpaf(short, long)]
    pub skin: String,
}

fn main() {
    let mut window = Window::new("Kiss3d: rectangle");
    let mut rect = window.add_rectangle(150.0, 150.0);
    rect.set_texture_from_file(Path::new("./a.png"), "a");

    //rect.set_color(0.0, 1.0, 0.0);

    let rot_rect = UnitComplex::new(0.014);

    while window.render() {
        rect.prepend_to_local_rotation(&rot_rect);
    }
}
