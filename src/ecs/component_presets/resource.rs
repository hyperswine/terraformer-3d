// RESOURCE COMPONENT

// Can be loadable with assetloader which uses this preset
// Textures must have a mesh assigned to it in the scene to be useful
// And meshes must have a model assigned to it in the scene to be useful

pub enum ResourceType {
    Texure,
    Mesh,
    AudioTrack,
}

/// A resource is a component that controls access to a single mutable piece of Data
/// That can be loaded from the filesystem
/// A resource stores the raw bytes of the data and encapsulates its type
pub struct Resource {
    resource_type: ResourceType,
    bytes: Vec<u8>,
}
