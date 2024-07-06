mod framebuffer;
mod line_impl;
mod bmp;

use framebuffer::Framebuffer;
use line_impl::Line;
use nalgebra_glm::Vec3;

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3]) {
    if vertices.len() < 3 {
        println!("Need at least 3 vertices to draw a polygon");
        return;
    }

    for i in 0..vertices.len() {
        let next = (i + 1) % vertices.len();
        let start = vertices[i];
        let end = vertices[next];

        // Convert from glm::Vec3 to (isize, isize) for framebuffer drawing
        framebuffer.Line(
            start.x as isize,
            start.y as isize,
            end.x as isize,
            end.y as isize,
        );
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Set the current drawing color to black
    framebuffer.set_current_color(0x000000);

    // Define vertices using nalgebra-glm's Vec3
    let vertices = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
        ];

    // Draw a polygon using the defined vertices
    draw_polygon(&mut framebuffer, &vertices);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("lines.bmp");
}