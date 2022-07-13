// -------------
// BASIC ANIMATIONS
// -------------

// Scaling, Rotation, Translation zigzags

// Splines using linear/cubic functions

// Splines using a discrete list of transform values for each timestep
// or specify seconds for each value as well

// Orbiting at a specific axis3d at radius r

// Scaling up by size k for n seconds then scaling back down
// Allow multiple splines, linear, cubic, or a custom discrete spline or function spline

// -------------
// IMPORTED ANIMATIONS
// -------------

// 2d plane based animations should be loaded as a gif

// 3d animations for a model or mesh should be loaded collectively
// as a .blend

use bevy_ecs::prelude::Component;

// Get the animation data from Model and isolate it here
// Use this to make changes or execute depending on game state and events
#[derive(Debug, Component)]
struct Animation3D {}
