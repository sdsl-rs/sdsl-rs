#[derive(Debug)]
pub struct Parameter {
    pub regex: String,
    pub capture_group_name: String,
    pub has_default: bool,
    pub is_sdsl_type: bool,
}

impl Parameter {
    pub fn integer(index: usize, has_default: bool) -> Self {
        let capture_group_name = get_capture_group_name(index);
        Self {
            regex: format!(
                r"(?P<{capture_group}>[0-9]*)_.*",
                capture_group = capture_group_name
            ),
            capture_group_name,
            has_default,
            is_sdsl_type: false,
        }
    }

    pub fn sdsl(index: usize, has_default: bool) -> Self {
        let capture_group_name = get_capture_group_name(index);
        Self {
            regex: format!(
                r"(?P<{capture_group}>sdsl::.*)",
                capture_group = capture_group_name
            ),
            capture_group_name,
            has_default,
            is_sdsl_type: true,
        }
    }
}

fn get_capture_group_name(index: usize) -> String {
    format!("i{index}", index = index)
}

pub trait Parameters {
    fn parameters(&self) -> Vec<Parameter>;
}
