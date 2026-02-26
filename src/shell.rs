pub struct Shell {
    pub last_exit_code: i32,
}

impl Shell {
    pub fn new() -> Self {
        Shell { last_exit_code: 0 }
    }
}
