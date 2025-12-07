pub type BeamIntensity = u64;
pub type BeamPosition = usize;
#[cfg(not(feature = "non-std-hash"))]
pub type BeamIntensityMap = std::collections::HashMap<BeamPosition, BeamIntensity>;
#[cfg(feature = "non-std-hash")]
pub type BeamIntensityMap = fxhash::FxHashMap<BeamPosition, BeamIntensity>;
pub type SplitterHitCount = u32;

pub fn default_intensity_map(capacity: usize) -> BeamIntensityMap {
    #[cfg(not(feature = "non-std-hash"))]
    {
        BeamIntensityMap::with_capacity(capacity)
    }
    #[cfg(feature = "non-std-hash")]
    {
        BeamIntensityMap::with_capacity_and_hasher(capacity, fxhash::FxBuildHasher::default())
    }
}
