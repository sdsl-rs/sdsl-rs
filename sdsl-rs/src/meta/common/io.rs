use crate::meta::common;
use anyhow::Result;

pub fn file_specifications(
    struct_c_code: &str,
    id: &str,
) -> Result<Vec<common::FileSpecification>> {
    let header = get_header_specification(&struct_c_code, &id)?;
    let source = get_source_specification(&header, &id)?;
    Ok(vec![header, source])
}

fn get_header_specification(struct_c_code: &str, id: &str) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("io.hpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: maplit::btreemap! {
            "#define IO_ID _id".to_string() => format!("#define IO_ID _{}", id),
            "#define STRUCTURE sdsl::int_vector<0>".to_string() => format!("#define STRUCTURE {}", struct_c_code),
        },
        template_file_name: template_file_name.clone(),
        target_file_name: target_file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn get_source_specification(
    header: &common::FileSpecification,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("io.cpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: maplit::btreemap! {
            format!("#include \"{}\"", header.template_file_name.display()) => format!("#include \"{}\"", header.target_file_name.display()),
        },
        template_file_name: template_file_name.clone(),
        target_file_name: target_file_name.clone(),
        c_file_type: common::CFileType::Cpp,
    })
}
