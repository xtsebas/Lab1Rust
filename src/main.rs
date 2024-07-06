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
    let vertices1 = vec![
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

    let vertices2 = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];    

    let vertices3 = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0),
    ];

    let vertices4 = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0),
    ];
    
    let vertices5 = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0),
    ];



    // Define the colors
    let line_color1 = 0xFFFFFF;
    let fill_color1 = 0xFFFF00;

    let line_color2 = 0xFFFFFF;
    let fill_color2 = 0x0000FF;
    
    let line_color3 = 0xFFFFFF;
    let fill_color3 = 0xFF0000;
    
    let line_color4 = 0xFFFFFF;
    let fill_color4 = 0x00FF00;

    let line_color5 = 0xFFFFFF;
    let fill_color5 = 0xFFFFFF;

    // Draw a polygon using the defined vertices
    draw_polygon(&mut framebuffer, &vertices1, line_color1);
    draw_polygon(&mut framebuffer, &vertices2, line_color2);
    draw_polygon(&mut framebuffer, &vertices3, line_color3);
    draw_polygon(&mut framebuffer, &vertices4, line_color4);
    draw_polygon(&mut framebuffer, &vertices5, line_color5);

    // Fill the polygon
    fill_polygon(&mut framebuffer, &vertices1, fill_color1);
    fill_polygon(&mut framebuffer, &vertices2, fill_color2);
    fill_polygon(&mut framebuffer, &vertices3, fill_color3);
    fill_polygon(&mut framebuffer, &vertices4, fill_color4);
    fill_polygon(&mut framebuffer, &vertices5, fill_color5);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("out.bmp");
}
