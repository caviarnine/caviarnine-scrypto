use crate::common::*;

pub fn setup() -> Result<(Store, TestEnvironment<InMemorySubstateDatabase>), RuntimeError> {
    let mut vars = Store::new();
    let mut api_base = TestEnvironment::new();
    let api = &mut api_base;
    let encoder = AddressBech32Encoder::for_simulator();
    
    let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None).mint_initial_supply(dec!(7), api)?;
    let owner_resource = owner_badge.resource_address(api)?;
    
    let fee_vaults_package = PackageFactory::compile_and_publish("../mock_fee_vaults", api, CompileProfile::Fast)?;
    let fee_vaults = FeeVaults::new(fee_vaults_package, api)?;
    let fee_vaults_component: ComponentAddress = fee_vaults.try_into().unwrap();
    
    let (wasm, package_definition) = Compile::compile_with_env_vars(
        this_package!(), 
        btreemap!(
            "FEE_VAULTS_PACKAGE".to_string() => fee_vaults_package.display(&encoder).to_string(),
            "FEE_VAULTS_COMPONENT".to_string() => fee_vaults_component.display(&encoder).to_string(),
            "OWNER_RESOURCE".to_string() => owner_resource.display(&encoder).to_string(),
        ),
        CompileProfile::Fast,
        false,
    );

    let (pool_package, _) = PackageFactory::publish(
        wasm,
        package_definition,
        MetadataInit::new(),
        api,
    )?;
    
    let token_x = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(8)
        .burn_roles(burn_roles! {
            burner => rule!(allow_all);
            burner_updater => rule!(deny_all);
        })
        .mint_initial_supply(dec!("100000000000"), api)?;
    let token_y = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(6)
        .burn_roles(burn_roles! {
            burner => rule!(allow_all);
            burner_updater => rule!(deny_all);
        })
        .mint_initial_supply(dec!("100000000000"), api)?;
    let token_z = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(18)
        .burn_roles(burn_roles! {
            burner => rule!(allow_all);
            burner_updater => rule!(deny_all);
        })
        .mint_initial_supply(dec!("100000000000"), api)?;

    let (pool, _lp_token) = WeightedPool::new_with_tokens(
        token_x.take(dec!(1000), api)?,
        token_y.take(dec!(1000), api)?,
        dec!(0.5), 
        dec!(0.01), 
        None,
        pool_package, 
        api, 
    )?;

    vars.store("owner_badge", owner_badge);
    vars.store("owner_resource", owner_resource);

    vars.store("fee_vaults_package", fee_vaults_package);
    vars.store("fee_vaults", fee_vaults);

    vars.store("pool_package", pool_package);
    vars.store("pool", pool);

    vars.store("token_x", token_x);
    vars.store("token_y", token_y);
    vars.store("token_z", token_z);

    Ok((vars, api_base))
}