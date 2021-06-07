// pub enum ParameterType {
//     Integer,
//     Sdsl,
// }

// #[derive(Debug)]
// struct Parameters {
//     parameters: Vec<Parameter>,
// }

// impl Parameters {
//     pub fn new() -> Self {
//         Self {
//             parameters: Vec::<_>::new(),
//         }
//     }

//     pub fn add(&mut self, parameter_type: &ParameterType, has_default: bool, c_index: usize) {
//         let index = self.parameters.len() + 1;
//         self.parameters.push(Parameter::new(&parameter_type, index, has_default, c_index ))
//     }
// }

#[derive(Debug)]
pub struct Parameter {
    pub regex: String,
    pub capture_group_name: String,
    pub has_default: bool,
    pub is_sdsl_type: bool,
    pub c_index: usize,
}

impl Parameter {
    // pub fn new(parameter_type: &ParameterType, index: usize, has_default: bool, c_index: usize) -> Self {
    //     match parameter_type {
    //         ParameterType::Integer => Self::integer(index, has_default, c_index),
    //         ParameterType::Sdsl => Self::sdsl(index, has_default, c_index),
    //     }
    // }

    pub fn integer(index: usize, has_default: bool, c_index: usize) -> Self {
        let capture_group_name = get_capture_group_name(index);
        Self {
            regex: format!(
                r"(?P<{capture_group}>[0-9]*)_.*",
                capture_group = capture_group_name
            ),
            capture_group_name,
            has_default,
            is_sdsl_type: false,
            c_index,
        }
    }

    pub fn sdsl(index: usize, has_default: bool, c_index: usize) -> Self {
        let capture_group_name = get_capture_group_name(index);
        Self {
            regex: format!(
                r"(?P<{capture_group}>sdsl::.*)",
                capture_group = capture_group_name
            ),
            capture_group_name,
            has_default,
            is_sdsl_type: true,
            c_index,
        }
    }
}

pub fn get_capture_group_name(index: usize) -> String {
    format!("i{index}", index = index)
}
