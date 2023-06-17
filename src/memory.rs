use uint256::Uint256;

pub struct Memory {
    store: Vec<u8>,
    last_gas_cost: u64,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            store: Vec::new(),
            last_gas_cost: 0,
        }
    }
    pub fn set(&mut self, offset: u64, size: u64, value: &[u8]) {
        if size > 0 {
            let end = offset + size;
            if end > self.store.len() as u64 {
                panic!("invalid memory: store empty");
            }
            self.store[offset as usize..end as usize].copy_from_slice(value);
        }
    }

    pub fn set32(&mut self, offset: u64, val: &Uint256) {
        let end = offset + 32;
        if end > self.store.len() as u64 {
            panic!("invalid memory: store empty");
        }
        let b32 = val.to_bytes32();
        self.store[offset as usize..end as usize].copy_from_slice(&b32[..]);
    }

    pub fn resize(&mut self, size: u64) {
        if self.store.len() < size as usize {
            self.store.resize(size as usize, 0);
        }
    }

    pub fn get_copy(&self, offset: i64, size: usize) -> Option<Vec<u8>> {
        if size == 0 || offset < 0 || offset >= self.store.len() as i64 {
            return None;
        }

        let end = (offset + size as i64).min(self.store.len() as i64);
        Some(self.store[offset as usize..end as usize].to_vec())
    }

    pub fn get_ptr(&self, offset: i64, size: usize) -> Option<&[u8]> {
        if size == 0 || offset < 0 || offset >= self.store.len() as i64 {
            return None;
        }

        let end = (offset + size as i64).min(self.store.len() as i64);
        Some(&self.store[offset as usize..end as usize])
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.store
    }
}
