// wrappers around rust physics

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BoundingBox3D {
    id: u64,
    min_axes: (f32, f32, f32),
    max_axes: (f32, f32, f32),
}

#[inline]
fn intersect(box_pair: &[BoundingBox3D; 2]) {}

use itertools::Itertools;

// if a collision is detected between at least one pair of objects, calculate the total momentum inherited by each for the next timestep
pub fn check_collision(objects: &[BoundingBox3D]) {
    let mut pairs = objects.into_iter().combinations(2);

    for pair in pairs {
        let obj1 = pair[0];
        let obj2 = pair[1];
        // check collision between obj1 and obj2
        if obj1.min_axes.0 <= obj2.max_axes.0
            && obj1.max_axes.0 >= obj2.min_axes.0
            && obj1.min_axes.1 <= obj2.max_axes.1
            && obj1.max_axes.1 >= obj2.min_axes.1
            && obj1.min_axes.0 <= obj2.max_axes.2
            && obj1.max_axes.0 >= obj2.min_axes.2
        {
            println!("Collision Detected!");
        }
    }
}

// A 3d collidable object with a bounding box
// pub trait CollisionMechanics3D {}

// * example implementation: CharacterObject
// struct CharacterObject {
//     bounding_box: BoundingBox3D
// }

// impl CollisionMechanics3D for CharacterObject {}
