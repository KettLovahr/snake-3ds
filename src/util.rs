pub fn distance_squared(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)
}
