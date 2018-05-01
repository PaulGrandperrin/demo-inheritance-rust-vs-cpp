#![feature(specialization)]
use std::marker::PhantomData;

// FlatObject

trait FlatObject: Drop {
    fn get_surface(&self) -> f32;
    fn get_thickness(&self) -> f32;

    fn get_volume(&self) -> f32 {
        println!("  computing volume from FlatObject");
        self.get_surface() * self.get_thickness()
    }

    fn destroy(&mut self);
}

// ThickObject

struct ThickObject<EXT> {
    thickness: f32,
    _e: EXT // _e(xtention) of type EXT(ention)
}

impl<EXT> ThickObject<EXT> {
    fn new_thick_object(thickness: f32, _e: EXT) -> Self {
        println!("  constructing ThickObject");
        Self {thickness, _e}
    }

    fn destroy_thick_object(&mut self) {
        println!("  destroying ThickObject");
    }
}

impl<EXT> FlatObject for ThickObject<EXT> {
    fn get_thickness(&self) -> f32 {
        println!("  accessing thickness");
        self.thickness
    }

    default fn get_surface(&self) -> f32 {
        unreachable!()
    }

    default fn destroy(&mut self) {
        self.destroy_thick_object()
    }
}

impl<EXT> Drop for ThickObject<EXT> {
    fn drop(&mut self) {
        println!("  dropping");
        self.destroy();
    }
}

// ThickCircle

struct Circle<EXT> {
    radius: f32,
    _e: EXT // _e(xtention) of type EXT(ention)
}
type ThickCircle<EXT> = ThickObject<Circle<EXT>>;

impl<EXT> ThickCircle<EXT> {
    fn new_thick_circle(thickness: f32, radius: f32, _e: EXT) -> Self {
        println!("  constructing ThickCircle");
        ThickObject::new_thick_object(thickness, Circle::<EXT>{radius, _e})
    }

    fn destroy_thick_circle(&mut self) {
        println!("  destroying ThickCircle");
    }
}

impl<EXT> FlatObject for ThickCircle<EXT> {
    fn get_surface(&self) -> f32 {
        println!("  computing surface from ThickCircle");
        std::f32::consts::PI * self._e.radius * self._e.radius
    }

    default fn destroy(&mut self) {
        self.destroy_thick_circle();
        self.destroy_thick_object();
    }
}

// ThickRectangle

struct Rectangle {
    height: f32,
    width: f32,
}
type ThickRectangle = ThickObject<Rectangle>;



impl ThickRectangle {
    fn new_thick_rectangle(thickness: f32, height: f32, width: f32) -> Self {
        println!("  constructing ThickRectangle");
        ThickObject::new_thick_object(thickness, Rectangle{height, width})
    }

    fn destroy_thick_rectangle(&mut self) {
        println!("  destroying ThickRectangle");
    }
}

impl FlatObject for ThickRectangle {
    fn get_surface(&self) -> f32 {
        println!("  computing surface from ThickRectangle");
        self._e.height * self._e.width
    }

    fn get_volume(&self) -> f32 { // overriding default method
        println!("  computing volume from ThickRectangle");
        self._e.height * self._e.width * self.thickness
    }

    fn destroy(&mut self) {
        self.destroy_thick_rectangle();
        self.destroy_thick_object();
    }
}

// generic function using static dispatch through monomorphization

fn print_volume(o: impl FlatObject) {
    let volume = o.get_volume();
    println!("volume: {}", volume);
}

fn main() {
    println!("ThickCircle of thickness 2 and radius 10");
    let c = ThickCircle::new_thick_circle(2., 10., PhantomData::<!>);
    print_volume(c); // using FlatObject::get_volume()
    println!();
    println!("ThickRectangle of thickness 3 and dimentions 2*4");
    let r = ThickRectangle::new_thick_rectangle(3., 2., 4.);
    print_volume(r); // using ThickRectangleTrait::get_volume()
}
