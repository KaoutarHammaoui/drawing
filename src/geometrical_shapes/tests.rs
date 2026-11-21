use super::*;
use raster::Image;

//Circle

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
        assert!(circle.raduis >= 1 && circle.raduis <= 50);
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


//Test for Point

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
        let mut image = Image::blank(10, 10);
        Point::new(5, 5).draw(&mut image);
        assert_eq!(image.get_pixel(5, 5).unwrap().r, 255);
    }

//Test for Triangle

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