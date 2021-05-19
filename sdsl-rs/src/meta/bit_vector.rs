use crate::meta::common;

pub fn file_specifications() -> Vec<common::FileSpecification> {
    vec![
        common::FileSpecification {
            replacements: maplit::btreemap! {},
            template_file_name: std::path::PathBuf::from("bit_vector.hpp"),
            target_file_name: std::path::PathBuf::from("bit_vector.hpp"),
            c_file_type: common::CFileType::Hpp,
        },
        common::FileSpecification {
            replacements: maplit::btreemap! {},
            template_file_name: std::path::PathBuf::from("bit_vector.cpp"),
            target_file_name: std::path::PathBuf::from("bit_vector.cpp"),
            c_file_type: common::CFileType::Cpp,
        },
    ]
}
