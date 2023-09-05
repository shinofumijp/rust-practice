pub struct HashMap {
    data: Vec<Vec<(usize, usize)>>,
    buckets: usize,
}

impl HashMap {
    const DEFAULT_BUCKET_SIZE: usize = 100;

    pub fn new() -> Self {
        Self::with_buckets(Self::DEFAULT_BUCKET_SIZE)
    }

    pub fn with_buckets(buckets: usize) -> Self {
        let data = vec![Vec::new(); buckets];
        Self { data, buckets }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.buckets
    }

    pub fn insert(&mut self, key: usize, val: usize) {
        let hashed = self.hash(key);
        match self.data[hashed].iter().position(|&(k, _)| k == key) {
            Some(index) => self.data[hashed][index] = (key, val),
            None => self.data[hashed].push((key, val)),
        }
    }

    pub fn contains(&mut self, key: usize) -> bool {
        let hashed: usize = self.hash(key);
        self.data[hashed].iter().any(|&(k, _)| k == key)
    }

    pub fn get(&mut self, key: usize) -> Option<usize> {
        let hashed: usize = self.hash(key);
        self.data[hashed]
            .iter()
            .find(|&&(k, _)| key == k)
            .map(|&(_, v)| v)
    }
}
