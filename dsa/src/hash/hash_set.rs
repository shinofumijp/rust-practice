#[derive(Debug)]
pub struct HashSet {
    data: Vec<Vec<usize>>,
    buckets: usize,
}

impl HashSet {
    const DEFAULT_BUCKET_SIZE: usize = 100;
    pub fn new() -> Self {
        Self::with_buckets(Self::DEFAULT_BUCKET_SIZE)
    }

    pub fn with_buckets(buckets: usize) -> Self {
        Self {
            data: vec![Vec::new(); buckets],
            buckets,
        }
    }

    fn hash(&self, value: usize) -> usize {
        value % self.buckets
    }

    pub fn insert(&mut self, value: usize) {
        let key = self.hash(value);
        if !self.data[key].contains(&value) {
            self.data[key].push(value);
        }
    }

    pub fn contains(&self, value: usize) -> bool {
        let key = self.hash(value);
        self.data[key].contains(&value)
    }
}
