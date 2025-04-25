use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use scrypto::prelude::*;

fn write_resource(var_name: &str, const_name: &str, decoder: &AddressBech32Decoder, file: &mut File) {
    let (var, resource) = env::var(var_name).map(|var| {
        let resource = ResourceAddress::try_from_bech32(&decoder, &var).unwrap();
        (Some(var), resource)
    }).unwrap_or_else(|_| {
        let resource = ResourceAddress::try_from_hex("5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6").unwrap();
        (None, resource)
    });
    writeln!(file, "pub const {}: ResourceAddress = ResourceAddress::new_or_panic({:?});", const_name, resource.into_node_id().to_vec()).unwrap();
    
    if let Some(var) = var {
        println!("cargo:warning={}: {}", var_name, var);
    }
}

fn write_package(var_name: &str, const_name: &str, decoder: &AddressBech32Decoder, file: &mut File) {
    let (var, package) = env::var(var_name).map(|var| {
        let package = PackageAddress::try_from_bech32(&decoder, &var).unwrap();
        (Some(var), package)
    }).unwrap_or_else(|_| {
        let package = PackageAddress::try_from_hex("0d89f83cb8550e3ec06cee7272b67d0855beff3a66bfbbfb33a127b5cfac").unwrap();
        (None, package)
    });
    writeln!(file, "pub const {}: PackageAddress = PackageAddress::new_or_panic({:?});", const_name, package.into_node_id().to_vec()).unwrap();
    
    if let Some(var) = var {
        println!("cargo:warning={}: {}", var_name, var);
    }
}

fn write_component(var_name: &str, const_name: &str, decoder: &AddressBech32Decoder, file: &mut File) {
    let (var, component) = env::var(var_name).map(|var| {
        let component = ComponentAddress::try_from_bech32(&decoder, &var).unwrap();
        (Some(var), component)
    }).unwrap_or_else(|_| {
        let component = ComponentAddress::try_from_hex("c169a00e3637d04099d059cb22912aaae58f08cdaf03139a3e10f40ac8cd").unwrap();
        (None, component)
    });
    writeln!(file, "pub const {}: ComponentAddress = ComponentAddress::new_or_panic({:?});", const_name, component.into_node_id().to_vec()).unwrap();
    
    if let Some(var) = var {
        println!("cargo:warning={}: {}", var_name, var);
    }
}

fn main() {
    // Determine the network to use
    let network = match env::var("NETWORK_ID").unwrap_or_default().as_str() {
        "1" => NetworkDefinition::mainnet(),
        "2" => NetworkDefinition::stokenet(),
        _ => NetworkDefinition::simulator(),
    };
    let decoder = AddressBech32Decoder::new(&network);

    // Specify the path for the output file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("env_constants.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Write constants to the output file
    write_resource("OWNER_RESOURCE", "_OWNER_RESOURCE", &decoder, &mut f);
    write_resource("LSULP_RESOURCE", "_LSULP_RESOURCE", &decoder, &mut f);

    write_package("FEE_VAULTS_PACKAGE", "FEE_VAULTS_PACKAGE", &decoder, &mut f);
    write_package("LSU_POOL_PACKAGE", "LSU_POOL_PACKAGE", &decoder, &mut f);

    write_component("FEE_VAULTS_COMPONENT", "_FEE_VAULTS_COMPONENT", &decoder, &mut f);
    write_component("LSU_POOL_COMPONENT", "_LSU_POOL_COMPONENT", &decoder, &mut f);
}
