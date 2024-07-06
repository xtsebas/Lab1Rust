mod framebuffer;
mod line_impl;
mod bmp;

use framebuffer::Framebuffer;
use line_impl::Line;
use nalgebra_glm::Vec3;

/// Draws a polygon by connecting the given vertices with lines.
/// The vertices must be provided in the order they are to be connected.
fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3], line_color: u32) {
    if vertices.len() < 3 {
        println!("Need at least 3 vertices to draw a polygon");
        return;
    }

    framebuffer.set_current_color(line_color);

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

/// Fills a polygon using the scanline algorithm.
fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3], fill_color: u32) {
    if vertices.len() < 3 {
        println!("Need at least 3 vertices to fill a polygon");
        return;
    }

    let mut edges: Vec<(isize, isize, isize, isize)> = Vec::new();

    for i in 0..vertices.len() {
        let next = (i + 1) % vertices.len();
        let start = vertices[i];
        let end = vertices[next];
        edges.push((start.x as isize, start.y as isize, end.x as isize, end.y as isize));
    }

    let min_y = vertices.iter().map(|v| v.y as isize).min().unwrap();
    let max_y = vertices.iter().map(|v| v.y as isize).max().unwrap();

    framebuffer.set_current_color(fill_color);

    for y in min_y..=max_y {
        let mut intersections: Vec<isize> = Vec::new();
        for &(x1, y1, x2, y2) in &edges {
            if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                let x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x);
            }
        }
        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                framebuffer.Line(intersections[i], y, intersections[i + 1], y);
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Define los vertices del poligono
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


    // Define the colors
    let line_color = 0xFFFFFF;
    let fill_color = 0xFFFF00;

    // Draw a polygon using the defined vertices
    draw_polygon(&mut framebuffer, &vertices, line_color);

    // Fill the polygon
    fill_polygon(&mut framebuffer, &vertices, fill_color);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("out.bmp");
}
