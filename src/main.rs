#![feature(specialization)]
use std::marker::PhantomData;

// FlatObject

trait FlatObjectTrait {
    fn get_surface(&self) -> f32;
    fn get_thickness(&self) -> f32;

    fn get_volume(&self) -> f32 {
        println!("  computing volume from FlatObject");
        self.get_surface() * self.get_thickness()
    }
}

// ThickObject

struct ThickObjectStruct<EXT> {
    thickness: f32,
    _e: EXT // _e(xtention) of type EXT(ention)
}

trait ThickObjectTrait<EXT>: FlatObjectTrait {
    fn new_thick_object(thickness: f32, _e: EXT) -> Self;
}

impl<EXT> ThickObjectTrait<EXT> for ThickObjectStruct<EXT> {
    fn new_thick_object(thickness: f32, _e: EXT) -> Self {
        println!("  constructing ThickObject");
        Self {thickness, _e}
    }
}

impl<EXT> FlatObjectTrait for ThickObjectStruct<EXT> {
    fn get_thickness(&self) -> f32 {
        println!("  accessing thickness");
        self.thickness
    }

    default fn get_surface(&self) -> f32 {
        unreachable!()
    }
}

// ThickCircle

struct CircleExt<EXT> {
    radius: f32,
    _e: EXT // _e(xtention) of type EXT(ention)
}
type ThickCircleStruct<EXT> = ThickObjectStruct<CircleExt<EXT>>;

trait ThickCircleTrait<EXT> {
    fn new_thick_circle(thickness: f32, radius: f32, _e: EXT) -> Self;
}

impl<EXT> ThickCircleTrait<EXT> for ThickCircleStruct<EXT> {
    fn new_thick_circle(thickness: f32, radius: f32, _e: EXT) -> Self {
        println!("  constructing ThickCircle");
        ThickObjectTrait::new_thick_object(thickness, CircleExt::<EXT>{radius, _e})
    }
}

impl<EXT> FlatObjectTrait for ThickCircleStruct<EXT> {
    fn get_surface(&self) -> f32 {
        println!("  computing surface from ThickCircle");
        std::f32::consts::PI * self._e.radius * self._e.radius
    }
}

// ThickRectangle

struct RectangleExt {
    height: f32,
    width: f32,
}
type ThickRectangleStruct = ThickObjectStruct<RectangleExt>;

trait ThickRectangleTrait {
    fn new_thick_rectangle(thickness: f32, height: f32, width: f32) -> Self;
}

impl ThickRectangleTrait for ThickRectangleStruct {
    fn new_thick_rectangle(thickness: f32, height: f32, width: f32) -> Self {
        println!("  constructing ThickRectangle");
        ThickObjectTrait::new_thick_object(thickness, RectangleExt{height, width})
    }
}

impl FlatObjectTrait for ThickRectangleStruct {
    fn get_surface(&self) -> f32 {
        println!("  computing surface from ThickRectangle");
        self._e.height * self._e.width
    }

    fn get_volume(&self) -> f32 { // overriding parent method
        println!("  computing volume from ThickRectangle");
        self._e.height * self._e.width * self.thickness
    }
}

// generic function using static dispatch through monomorphization

fn print_volume(o: impl FlatObjectTrait) {
    let volume = o.get_volume();
    println!("volume: {}", volume);
}

fn main() {
    println!("ThickCircle of thickness 2 and radius 10");
    let c = ThickCircleStruct::new_thick_circle(2., 10., PhantomData::<!>);
    print_volume(c); // using FlatObjectTrait::get_volume()
    println!();
    println!("ThickRectangle of thickness 3 and dimentions 2*4");
    let r = ThickRectangleStruct::new_thick_rectangle(3., 2., 4.);
    print_volume(r); // using ThickRectangleTrait::get_volume()
}
