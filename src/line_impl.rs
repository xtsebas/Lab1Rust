use crate::framebuffer::Framebuffer;

pub trait Line {
    fn Line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize);
}

impl Line for Framebuffer {
    fn Line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.point(x, y);
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                if x == x2 {
                    break;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 {
                    break;
                }
                err += dx;
                y += sy;
            }
        }
    }
}