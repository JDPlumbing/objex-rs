pub mod core;
pub mod systems;

// Explicit re-exports from `core`
pub use core::{
    Object,
    CompositeObject,
};

// Explicit re-exports from `systems`
pub use systems::{
    mass,
    strength,
    thermal,
    degradation,
    mechanical,
    composite as systems_composite, // renamed to avoid conflict with core::composite
    electrical,
};
