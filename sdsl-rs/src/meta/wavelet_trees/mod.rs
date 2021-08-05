use anyhow::Result;

pub mod layouts;
pub mod wt_huff;
pub mod wt_int;

pub fn get_metas() -> Result<Vec<Box<dyn crate::meta::common::Meta>>> {
    let mut metas = vec![
        Box::new(wt_huff::WtHuffMeta::new()) as Box<dyn crate::meta::common::Meta>,
        Box::new(wt_int::WtIntMeta::new()) as Box<dyn crate::meta::common::Meta>,
    ];
    metas.extend(layouts::get_metas()?);
    Ok(metas)
}
