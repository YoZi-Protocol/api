#[cfg(feature = "cache")]
mod moka;

#[cfg(feature = "cache")]
pub use moka::CacheService;

#[cfg(not(feature = "cache"))]
mod mock;

#[cfg(not(feature = "cache"))]
pub use mock::CacheService;
