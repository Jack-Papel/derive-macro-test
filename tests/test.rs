#![allow(dead_code)]

use std::fmt::Debug;
mod common;
use common::*;

use derive_macro_test::Transformable;

#[derive(Transformable, Debug)]
struct Drone {
    #[transform]
    transform: Transform,
    tasks: Vec<()>,
}

#[derive(Transformable, Debug)]
struct Metal {
    #[transform]
    transform: Transform,
}

#[test]
fn test() {
    let mut entities: Vec<Box<dyn Entity>> = vec![];

    let drone = Drone {
        transform: Transform(0.0, 0.0),
        tasks: vec![],
    };
    println!("transform: {:#?}", drone.transform());

    entities.push(Box::new(drone));
    entities.push(Box::new(Metal {
        transform: Transform(1.0, 0.0),
    }));

    println!("entities: {:#?}", entities);
}
