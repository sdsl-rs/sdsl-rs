use anyhow::Result;

pub mod params;

#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub enum CFileType {
    Cpp,
    Hpp,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub struct FileSpecification {
    pub replacements: std::collections::BTreeMap<String, String>,
    pub template_file_name: std::path::PathBuf,
    pub target_file_name: std::path::PathBuf,
    pub c_file_type: CFileType,
}

pub trait Meta: Regex + Path + params::Parameters {
    fn file_specifications(
        &self,
        parameter_values: &Option<&Vec<String>>,
        id: &str,
    ) -> Result<Vec<FileSpecification>>;
}

pub trait Path {
    fn path(&self) -> String;
}

pub trait Regex: Path + params::Parameters {
    fn parameters_regex(&self) -> Result<Option<Vec<regex::Regex>>>;
    fn default_regex(&self) -> Result<Option<regex::Regex>>;
}

impl<T: Path + params::Parameters> Regex for T {
    /// Return regex for structure with generic parameters.
    ///
    /// Returns None if structure does not include generic parameters.
    fn parameters_regex(&self) -> Result<Option<Vec<regex::Regex>>> {
        let parameters = self.parameters();
        if parameters.is_empty() {
            return Ok(None);
        }

        let get_regex = |parameters: &[params::Parameter]| -> Result<regex::Regex> {
            let parameters_regex = parameters
                .iter()
                .map(|p| p.regex.clone())
                .collect::<Vec<String>>()
                .join(r"\s*,\s*");
            let structure_regex = format!(
                r".*{path}<{params}>;.*",
                path = self.path(),
                params = parameters_regex
            );
            Ok(regex::Regex::new(&structure_regex)?)
        };

        let mut regexes = vec![get_regex(&parameters)?];
        for (index, parameter) in parameters.iter().rev().enumerate() {
            if parameter.has_default {
                let parameters_subset = &parameters[..parameters.len() - index];
                regexes.push(get_regex(parameters_subset)?);
            }
        }

        Ok(Some(regexes))
    }

    /// Return regex for structure without generic parameters
    /// or where all paramaters have default values.
    ///
    /// Returns None if structure has generic parameters without default.
    fn default_regex(&self) -> Result<Option<regex::Regex>> {
        // The resultant regex will not capture anything unless
        // all parameters have default values.
        if !self.parameters().iter().all(|p| p.has_default) {
            return Ok(None);
        }

        let structure_regex = format!(r".*{path};.*", path = self.path(),);
        Ok(Some(regex::Regex::new(&structure_regex)?))
    }
}
