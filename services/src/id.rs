use fastid::FastIdWorker;

pub struct IdService(FastIdWorker);

impl IdService {
    pub fn new(machine_id: u64) -> Self {
        Self(FastIdWorker::with_bits_and_epoch(
            40,
            16,
            7,
            machine_id,
            1672531200000000000,
        ))
    }

    pub fn next_id(&self) -> i64 {
        self.0.next_id().as_i64()
    }
}
