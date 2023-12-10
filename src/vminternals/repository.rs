use fnv::FnvHashMap;
pub struct VMRepository {
    pub repo_var_pointers: FnvHashMap<usize, usize>,
    pub repo_capacity: usize,
}

impl VMRepository {
    pub fn new(repo_capacity: usize) -> Self {
        VMRepository {
            repo_var_pointers: FnvHashMap::with_capacity_and_hasher(
                repo_capacity,
                Default::default(),
            ),
            repo_capacity,
        }
    }

    pub fn add_var(&mut self, var_name: usize, var_data: usize) {
        if self.repo_capacity == self.repo_var_pointers.len() {
            panic!("[ HEAP OVERFLOW ]")
        }

        self.repo_var_pointers.insert(var_name, var_data);
    }

    pub fn remove_var(&mut self, var_name: usize) {
        if !self.repo_var_pointers.contains_key(&var_name) {
            panic!("[ UNDEFINED VARIABLE ADDRESS ]")
        }

        self.repo_var_pointers.remove(&var_name);
    }

    pub fn get_var(&mut self, var_name: usize) -> &usize {
        self.repo_var_pointers.get(&var_name).unwrap()
    }

    pub fn pop_var(&mut self, var_name: usize) -> usize {
        if 0 == self.repo_var_pointers.len() {
            panic!("[ HEAP UNDERFLOW ]")
        }

        self.repo_var_pointers.remove(&var_name).unwrap()
    }

    pub fn clear_heap(&mut self) {
        self.repo_var_pointers.clear();
    }
}
