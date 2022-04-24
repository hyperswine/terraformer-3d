// ------------
// DECORATION
// ------------

// a non "interactable" object, e.g. no interface that pops up to let player interact and do something interesting with it
// simply place it somewhere and move it through worldphysics
// like collisions
// on a real physics eng, everything should be 'moveable'
// albeit very slightly for large masses like cars, trees and buildings with total mesh mass > tons
pub struct Decoration {
    mass: f32,
}
