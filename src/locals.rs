thread_local! {
    pub static NUM_CORES: usize = std::thread::available_parallelism().unwrap().get();
}