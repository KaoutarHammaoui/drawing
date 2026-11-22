use super::*;
use raster::Image;

//Circle
mod circle_tests {
    use super::*;
    #[test]
    fn zero_radius_draws_one_pixel() {
        let mut image = Image::blank(10, 10);
        let before = image.get_pixel(5, 5).unwrap();

        let circle = Circle::new(&Point::new(5, 5), 0);
        circle.draw(&mut image);

        let after = image.get_pixel(5, 5).unwrap();

        assert!(before.r != after.r || before.g != after.g || before.b != after.b);
    }

    #[test]
    fn random_circle_has_valid_center() {
        for _ in 0..1000 {
            let circle = Circle::random(100, 100);

            assert!(circle.center.x >= 0 && circle.center.x < 100);
            assert!(circle.center.y >= 0 && circle.center.y < 100);
            assert!(circle.raduis >= 20 && circle.raduis <= 550);
        }
    }

    #[test]
    fn circle_draws_symmetric_pixels() {
        let mut image = Image::blank(30, 30);
        let circle = Circle::new(&Point::new(10, 10), 5);

        circle.draw(&mut image);

        let points = [(10, 15), (10, 5), (15, 10), (5, 10)];

        let first = image.get_pixel(10, 15).unwrap();

        for (x, y) in points {
            let pixel = image.get_pixel(x, y).unwrap();

            assert_eq!(pixel.r, first.r);
            assert_eq!(pixel.g, first.g);
            assert_eq!(pixel.b, first.b);
        }
    }
}

//Test for Point

mod poit_tests {
    use super::*;

    //tell rust that this is a test function
    #[test]
    fn random_stays_inside_image() {
        for _ in 0..1000 {
            let p = Point::random(100, 50);
            assert!(p.x >= 0 && p.x < 100);
            assert!(p.y >= 0 && p.y < 50);
        }
    }

    #[test]
    //compare the values of x and y with the expected values
    fn new_keeps_values() {
        let p = Point::new(3, 4);
        assert_eq!((p.x, p.y), (3, 4));
    }

    #[test]
    //check if the draw function sets the pixel at the correct position
    fn draw_sets_one_pixel() {
    let blank = Image::blank(10, 10);
    let mut image = Image::blank(10, 10);
    Point::new(5, 5).draw(&mut image);

    let p = image.get_pixel(5, 5).unwrap();
    let b = blank.get_pixel(5, 5).unwrap();
    assert!((p.r, p.g, p.b) != (b.r, b.g, b.b));
    }
}

//Test for Triangle

mod triangle_tests {
    use super::*;

    fn drawn(image: &Image, blank: &Image, x: i32, y: i32) -> bool {
        let p = image.get_pixel(x, y).unwrap();
        let b = blank.get_pixel(x, y).unwrap();

        (p.r, p.g, p.b) != (b.r, b.g, b.b)
    }

    #[test]
    fn draws_its_corners() {
        let blank = Image::blank(10, 10);
        let mut image = Image::blank(10, 10);
        Triangle::new(&Point::new(1, 1), &Point::new(8, 1), &Point::new(4, 8)).draw(&mut image);

        for (x, y) in [(1, 1), (8, 1), (4, 8)] {
            assert!(drawn(&image, &blank, x, y));
        }
    }

    #[test]
    fn reverse_order_still_works() {
        // same triangle, points given in reverse order
        let blank = Image::blank(10, 10);
        let mut image = Image::blank(10, 10);
        Triangle::new(&Point::new(4, 8), &Point::new(8, 1), &Point::new(1, 1)).draw(&mut image);

        for (x, y) in [(1, 1), (8, 1), (4, 8)] {
            assert!(drawn(&image, &blank, x, y));
        }
    }

    #[test]
    fn same_coordinates_draws_one_pixel() {
        let p = Point::new(3, 3);
        let blank = Image::blank(10, 10);
        let mut image = Image::blank(10, 10);
        Triangle::new(&p, &p, &p).draw(&mut image);

        assert!(drawn(&image, &blank, 3, 3));
    }
}

//Tests for Line

mod line_tests {
    use super::*;

    #[test]
    fn test_line_same_coordinates() {
        let point = Point::new(3, 3);

        let line = Line::new(&point, &point);

        assert_eq!(line.a.x, 3);
        assert_eq!(line.a.y, 3);
        assert_eq!(line.b.x, 3);
        assert_eq!(line.b.y, 3);
    }

    #[test]
    fn test_random_line_stays_within_bounds() {
        let width = 100;
        let height = 80;

        for _ in 0..1000 {
            let line = Line::random(width, height);

            assert!(line.a.x >= 0 && line.a.x < width);
            assert!(line.a.y >= 0 && line.a.y < height);

            assert!(line.b.x >= 0 && line.b.x < width);
            assert!(line.b.y >= 0 && line.b.y < height);
        }
    }
}

//Tests for rectangle

mod rectangle_tests {
    use super::*;

    #[test]
    fn test_rectangle_with_inverted_coordinates() {
        let a = Point::new(150, 300);
        let b = Point::new(50, 60);

        let rectangle = Rectangle::new(&a, &b);

        assert_eq!(rectangle.a.x, 50);
        assert_eq!(rectangle.a.y, 60);

        assert_eq!(rectangle.b.x, 150);
        assert_eq!(rectangle.b.y, 300);
    }

    #[test]
    fn test_rectangle_with_mixed_coordinates() {
        let a = Point::new(150, 60);
        let b = Point::new(50, 300);

        let rectangle = Rectangle::new(&a, &b);

        assert_eq!(rectangle.a.x, 50);
        assert_eq!(rectangle.a.y, 60);

        assert_eq!(rectangle.b.x, 150);
        assert_eq!(rectangle.b.y, 300);
    }
}
