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
    
    let lsu_pool_package = PackageFactory::compile_and_publish("../mock_lsu_pool", api, CompileProfile::Fast)?;
    let lsu_pool = LsuPool::new(dec!(110000000000), lsu_pool_package, api)?;
    let lsu_pool_component: ComponentAddress = lsu_pool.try_into().unwrap();

    let token_lsulp = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(8)
        .burn_roles(burn_roles! {
            burner => rule!(allow_all);
            burner_updater => rule!(deny_all);
        })
        .mint_initial_supply(dec!("100000000000"), api)?;

    api.disable_auth_module();
    let token_xrd = ResourceManager(XRD).mint_fungible(dec!(100000000000), api)?;
    api.enable_auth_module();

    let token_z = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(18)
        .burn_roles(burn_roles! {
            burner => rule!(allow_all);
            burner_updater => rule!(deny_all);
        })
        .mint_initial_supply(dec!("100000000000"), api)?;

    let (wasm, package_definition) = Compile::compile_with_env_vars(
        this_package!(), 
        btreemap!(
            "FEE_VAULTS_PACKAGE".to_string() => fee_vaults_package.display(&encoder).to_string(),
            "FEE_VAULTS_COMPONENT".to_string() => fee_vaults_component.display(&encoder).to_string(),
            "LSU_POOL_PACKAGE".to_string() => lsu_pool_package.display(&encoder).to_string(),
            "LSU_POOL_COMPONENT".to_string() => lsu_pool_component.display(&encoder).to_string(),
            "LSULP_RESOURCE".to_string() => token_lsulp.resource_address(api)?.display(&encoder).to_string(),
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



    let (pool, _lp_token) = HyperStake::new_with_tokens(
        token_lsulp.take(dec!(200), api)?,
        token_xrd.take(dec!(1000), api)?,
        dec!(1.02), 
        dec!(0.90), 
        dec!(0.01), 
        None,
        pool_package, 
        api, 
    )?;

    vars.store("owner_badge", owner_badge);
    vars.store("owner_resource", owner_resource);

    vars.store("fee_vaults_package", fee_vaults_package);
    vars.store("fee_vaults", fee_vaults);

    vars.store("lsu_pool_package", lsu_pool_package);
    vars.store("lsu_pool", lsu_pool);

    vars.store("pool_package", pool_package);
    vars.store("pool", pool);

    vars.store("token_lsulp", token_lsulp);
    vars.store("token_xrd", token_xrd);
    vars.store("token_z", token_z);

    Ok((vars, api_base))
}