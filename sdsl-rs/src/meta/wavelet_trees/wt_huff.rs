use crate::meta::common::{self, Code, Parameters};
use anyhow::{format_err, Result};

type DefaultInterfaceType<'a> = crate::interface::wavelet_trees::wt_huff::WtHuff<
    'a,
    crate::interface::bit_vectors::bit_vector::BitVector,
>;

pub struct WtHuffMeta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl WtHuffMeta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![
                Box::new(crate::meta::bit_vectors::bit_vector::BitVectorMeta::new()) as Box<dyn common::Meta>,
                Box::new(
                    crate::meta::rank_support_v::RankSupportVMeta::new_parameterized(vec![Box::new(
                        crate::meta::common::bit_patterns::P1Meta::new(),
                    )
                        as Box<dyn common::Meta>]),
                ) as Box<dyn common::Meta>,
                Box::new(
                    crate::meta::select_support_mcl::SelectSupportMclMeta::new_parameterized(vec![
                        Box::new(crate::meta::common::bit_patterns::P1Meta::new())
                            as Box<dyn common::Meta>,
                    ]),
                ) as Box<dyn common::Meta>,
                Box::new(
                    crate::meta::select_support_mcl::SelectSupportMclMeta::new_parameterized(vec![
                        Box::new(crate::meta::common::bit_patterns::P0Meta::new())
                            as Box<dyn common::Meta>,
                    ]),
                ) as Box<dyn common::Meta>,
                Box::new(
                    crate::meta::wavelet_trees::layouts::byte_tree::ByteTreeMeta::new_parameterized(vec![
                        Box::new(crate::meta::wavelet_trees::layouts::common::BreadthFirstSearchMeta::new())
                            as Box<dyn common::Meta>,
                    ]),
                ) as Box<dyn common::Meta>,
            ]
        }
    }
}

impl common::Meta for WtHuffMeta {
    fn file_specifications(
        &self,
        parameters_c_code: &Vec<String>,
        parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = header_specification(&parameters_c_code, &parameters_file_specs, &id, &self)?;
        let source = source_specification(&header, &id)?;

        let bit_vector_meta = crate::meta::bit_vectors::bit_vector::BitVectorMeta::new();
        let bit_vector_specs = bit_vector_meta.file_specifications(&vec![], &vec![], &id)?;

        let c_code = self.c_code(&parameters_c_code)?;
        let io_specifications = common::io::file_specifications(&c_code, None, &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(bit_vector_specs);
        specifications.extend(io_specifications);

        let tree_strategy_file_specs = parameters_file_specs.last().ok_or(format_err!(
            "Parameters file specs empty. Expected at least one element."
        ))?;
        let tree_strategy_header_spec = tree_strategy_file_specs.first().ok_or(format_err!(
            "Parameter file specs empty. Expected at least one element."
        ))?;
        specifications.push(tree_strategy_header_spec.clone());

        Ok(specifications)
    }
}

fn header_specification(
    parameters_c_code: &Vec<String>,
    parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
    id: &str,
    meta: &WtHuffMeta,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("wt_huff.hpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: get_header_replacements(
            &parameters_c_code,
            parameters_file_specs,
            &id,
            &meta,
        )?,
        template_file_name: template_file_name.clone(),
        target_file_name: target_file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn source_specification(
    header: &common::FileSpecification,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("wt_huff.cpp");
    Ok(common::FileSpecification {
        replacements: get_source_replacements(&header.template_file_name, &header.target_file_name),
        template_file_name: template_file_name.clone(),
        target_file_name: common::get_target_file_name(&template_file_name, &id)?,
        c_file_type: common::CFileType::Cpp,
    })
}

fn get_source_replacements(
    header_template_file_name: &std::path::PathBuf,
    header_target_file_name: &std::path::PathBuf,
) -> std::collections::BTreeMap<String, String> {
    maplit::btreemap! {
        format!("#include \"{}\"", header_template_file_name.display()) => format!("#include \"{}\"", header_target_file_name.display())
    }
}

fn get_header_replacements(
    parameters_c_code: &Vec<String>,
    parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
    id: &str,
    meta: &WtHuffMeta,
) -> Result<std::collections::BTreeMap<String, String>> {
    let mut replacements = maplit::btreemap! {};

    let parameters = meta.parameters_definitions();
    let parameters_c_code = common::c_sorted_parameters(&parameters_c_code, &parameters)?;
    replacements.insert(
        "#define WT_HUFF_TEMPLATE sdsl::bit_vector, sdsl::bit_vector::rank_1_type, sdsl::bit_vector::select_1_type, sdsl::bit_vector::select_0_type, sdsl::byte_tree<>".to_string(),
        format!(
            "#define WT_HUFF_TEMPLATE {}",
            parameters_c_code.join(", ")
        ),
    );

    replacements.insert(
        "#define WT_HUFF_ID _id".to_string(),
        format!("#define WT_HUFF_ID _{}", id),
    );

    let tree_strategy_file_specs = parameters_file_specs.last().ok_or(format_err!(
        "Parameters file specs empty. Expected at least one element."
    ))?;
    let tree_strategy_header_spec = tree_strategy_file_specs.first().ok_or(format_err!(
        "Parameter file specs empty. Expected at least one element."
    ))?;
    replacements.insert(
        "#include \"byte_tree.hpp\"".to_string(),
        format!(
            "#include \"{}\"",
            tree_strategy_header_spec.target_file_name.display()
        ),
    );

    Ok(replacements)
}

impl common::Path for WtHuffMeta {
    fn path(&self) -> String {
        "sdsl::wavelet_trees::WtHuff".to_string()
    }
}

impl common::Code for WtHuffMeta {
    fn c_code(&self, parameters_c_code: &Vec<String>) -> Result<String> {
        let parameters = self.parameters_definitions();
        let parameters_c_code = common::c_sorted_parameters(&parameters_c_code, &parameters)?;
        Ok(format!("sdsl::wt_huff<{}>", parameters_c_code.join(", ")))
    }
}

impl common::Parameters for WtHuffMeta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![
            common::params::Parameter::sdsl(0, true, 0),
            common::params::Parameter::sdsl(1, true, 1),
            common::params::Parameter::sdsl(2, true, 2),
            common::params::Parameter::sdsl(3, true, 3),
            common::params::Parameter::sdsl(4, true, 4),
        ]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        use crate::interface::common::Code;
        DefaultInterfaceType::parameters_c_code()
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}
