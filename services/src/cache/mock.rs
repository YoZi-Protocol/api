use std::{future::Future, hash::Hash, marker::PhantomData};

#[di::injectable]
#[derive(Clone)]
pub struct CacheService<T: Clone + Send + Sync + 'static> {
    phantom: PhantomData<T>,
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
        f.await
    }

    pub async fn invalidate<K: Hash>(&self, key: K) {}
}
