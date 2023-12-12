use fnv::FnvHashMap;

/// ## Repository struct implementation.
///
/// Used mostly for global variables.
pub struct VMRepository {
    /// Contains pointers to the heap
    pub repo_var_pointers: FnvHashMap<usize, usize>,

    /// Contains the repository's capacity
    pub repo_capacity: usize,
}

impl VMRepository {
    /// Instantiates the VMRepository struct and returns it.
    pub fn new(repo_capacity: usize) -> Self {
        VMRepository {
            repo_var_pointers: FnvHashMap::with_capacity_and_hasher(
                repo_capacity,
                Default::default(),
            ),
            repo_capacity,
        }
    }

    /// Add a variable to the repository, containing the "name"/address,
    /// and the data/pointer (The pointer points to the heap);
    pub fn add_var(&mut self, var_name: usize, var_data: usize) {
        if self.repo_capacity == self.repo_var_pointers.len() {
            panic!("[ HEAP OVERFLOW ]")
        }

        self.repo_var_pointers.insert(var_name, var_data);
    }

    /// Gets variable pointer from the repository.
    pub fn get_var(&mut self, var_name: usize) -> &usize {
        self.repo_var_pointers
            .get(&var_name)
            .expect("Variable don't exist!")
    }

    /// Pops variable from the repository and return it's pointer.
    pub fn pop_var(&mut self, var_name: usize) -> usize {
        if self.repo_var_pointers.is_empty() {
            panic!("[ HEAP UNDERFLOW ]")
        }

        self.repo_var_pointers
            .remove(&var_name)
            .expect("Variable don't exist!")
    }

    /// (_Advanced_): Removes variable from the repository.
    pub fn remove_var(&mut self, var_name: usize) {
        if !self.repo_var_pointers.contains_key(&var_name) {
            panic!("[ UNDEFINED VARIABLE ADDRESS ]")
        }

        self.repo_var_pointers.remove(&var_name);
    }

    /// (_Advanced_): Removes everything from the repository.
    pub fn clear_repo(&mut self) {
        self.repo_var_pointers.clear();
    }
}
