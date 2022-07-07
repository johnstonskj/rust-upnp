use crate::discovery::{ProductVersion, ProductVersions};
use crate::SpecVersion;

pub fn user_agent_string(spec_version: SpecVersion, product: &Option<ProductVersion>) -> String {
    let versions = ProductVersions::new(
        ProductVersion::for_upnp_version(spec_version),
        if let Some(product) = product {
            product.clone()
        } else {
            ProductVersion::for_default_product()
        },
        ProductVersion::for_platform(),
    );
    versions.to_string()
}
