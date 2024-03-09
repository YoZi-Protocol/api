use std::{
    collections::hash_map::RandomState,
    future::Future,
    hash::{BuildHasher, Hash},
    ops::Deref,
    sync::Arc,
    time::Duration,
};

use moka_cache::future::Cache;

#[derive(Clone)]
struct Moka<T: Clone + Send + Sync + 'static>(Arc<Cache<u64, T>>);

impl<T> Default for Moka<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        let moka = Cache::builder()
            .max_capacity(1200)
            .time_to_live(Duration::from_secs(30 * 60))
            .time_to_idle(Duration::from_secs(5 * 60))
            .build();

        Self(Arc::new(moka))
    }
}

impl<T> AsRef<Cache<u64, T>> for Moka<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn as_ref(&self) -> &Cache<u64, T> {
        &self.0
    }
}

impl<T> Deref for Moka<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Target = Cache<u64, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[di::injectable]
#[derive(Clone)]
pub struct CacheService<T: Clone + Send + Sync + 'static> {
    cache: Moka<T>,
    build_hasher: RandomState,
}

impl<T> CacheService<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub async fn get_with<K: Hash, F: Future<Output = Option<T>>>(
        &self,
        key: K,
        f: F,
    ) -> Option<T> {
        let key = self.build_hasher.hash_one(&key);
        self.cache.optionally_get_with(key, f).await
    }

    pub async fn invalidate<K: Hash>(&self, key: K) {
        let key = self.build_hasher.hash_one(&key);

        self.cache.invalidate(&key).await
    }
}
