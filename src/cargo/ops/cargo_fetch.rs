use std::path::Path;

use core::registry::PackageRegistry;
use core::{Package, PackageId, Resolve};
use ops;
use util::{CargoResult, Config, human, ChainError};

pub struct FetchOptions<'a>{
    pub config: &'a Config,
    pub network_retry: u32,
}

/// Executes `cargo fetch`.
pub fn fetch(manifest_path: &Path, opts: &FetchOptions) -> CargoResult<()> {
    let package = try!(Package::for_path(manifest_path, opts.config));
    let mut registry = PackageRegistry::new(opts.config, opts.network_retry);
    let resolve = try!(ops::resolve_pkg(&mut registry, &package));
    let _ = try!(get_resolved_packages(&resolve, &mut registry));
    Ok(())
}

pub fn get_resolved_packages(resolve: &Resolve, registry: &mut PackageRegistry)
                             -> CargoResult<Vec<Package>> {
    let ids: Vec<PackageId> = resolve.iter().cloned().collect();
    registry.get(&ids).chain_error(|| {
        human("unable to get packages from source")
    })
}
