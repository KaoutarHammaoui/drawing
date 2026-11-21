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
