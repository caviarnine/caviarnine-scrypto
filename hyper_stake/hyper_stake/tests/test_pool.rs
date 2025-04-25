mod common;

use common::*;

#[test]
fn test_new() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");

    // Act
    let pool = HyperStake::new(
        dec!(1.001),
        dec!(1),
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(0));
    assert_eq!(pool_info.resource_x, token_lsulp.resource_address(env)?);
    assert_eq!(pool_info.resource_y, token_xrd.resource_address(env)?);
    assert_eq!(pool_info.reserve_x, dec!(0));
    assert_eq!(pool_info.reserve_y, dec!(0));
    assert_eq!(pool_info.oracle_price, dec!(1.1));
    assert_eq!(pool_info.upper_offset, dec!(1.001));
    assert_eq!(pool_info.lower_offset, dec!(1));
    assert_eq!(pool_info.fee, dec!(0.001));
    assert_eq!(pool_info.protocol_fee_share, dec!(0.1));
    assert_eq!(pool_info.treasury_fee_share, dec!(0.1));

    Ok(())
}

#[test]
fn test_new_with_tokens() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");

    // Act
    let (pool, lp_token) = HyperStake::new_with_tokens(
        token_lsulp.take(dec!(0), env)?,
        token_xrd.take(dec!(1000), env)?,
        dec!(1.001), 
        dec!(1), 
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert!((pool_info.price - dec!(1.1) * dec!(1.001)).checked_abs().unwrap() < dec!(0.00000000000000001));
    assert_eq!(pool_info.resource_x, token_lsulp.resource_address(env)?);
    assert_eq!(pool_info.resource_y, token_xrd.resource_address(env)?);
    assert_eq!(pool_info.reserve_x, dec!(0));
    assert_eq!(pool_info.reserve_y, dec!(1000));
    assert_eq!(pool_info.oracle_price, dec!(1.1));
    assert_eq!(pool_info.upper_offset, dec!(1.001));
    assert_eq!(pool_info.lower_offset, dec!(1));
    assert_eq!(pool_info.fee, dec!(0.001));
    assert_eq!(pool_info.protocol_fee_share, dec!(0.1));
    assert_eq!(pool_info.treasury_fee_share, dec!(0.1));
    assert_eq!(pool_info.lp_resource, lp_token.resource_address(env)?);

    Ok(())
}

#[test]
fn test_change_oracle_price() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut lsu_pool = vars.fetch::<LsuPool>("lsu_pool");
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");

    let (pool, _lp_token) = HyperStake::new_with_tokens(
        token_lsulp.take(dec!(100), env)?,
        token_xrd.take(dec!(100), env)?,
        dec!(1.1), 
        dec!(0.9), 
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Act
    lsu_pool.set_dex_valuation_xrd(dec!(120000000000), env)?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(1.183696644276083431));

    // Act
    lsu_pool.set_dex_valuation_xrd(dec!(130000000000), env)?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(1.277380596229710598));

    // Act
    lsu_pool.set_dex_valuation_xrd(dec!(140000000000), env)?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(1.370760583782313921));

    // Act
    lsu_pool.set_dex_valuation_xrd(dec!(150000000000), env)?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(1.463873318051934813));

    Ok(())
}

#[test]
fn test_set_protocol_fee_share() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    env.disable_auth_module();
    pool.set_protocol_fee_share(dec!(0), env)?;
    env.enable_auth_module();

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.protocol_fee_share, dec!(0));

    Ok(())
}

#[test]
fn test_set_protocol_fee_share_invalid() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    env.disable_auth_module();
    let result = pool.set_protocol_fee_share(dec!(0.11), env);
    env.enable_auth_module();
    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_set_protocol_fee_share_auth() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let result = pool.set_protocol_fee_share(dec!(0), env);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_set_treasury_fee_share() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    env.disable_auth_module();
    pool.set_treasury_fee_share(dec!(0), env)?;
    env.enable_auth_module();

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.treasury_fee_share, dec!(0));

    Ok(())
}

#[test]
fn test_set_treasury_fee_share_invalid() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    env.disable_auth_module();
    let result = pool.set_treasury_fee_share(dec!(0.11), env);
    env.enable_auth_module();
    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_set_treasury_fee_share_auth() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let result = pool.set_treasury_fee_share(dec!(0), env);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_add_liquidity() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let (lp_token, remainder) = pool.add_liquidity(
        token_lsulp.take(dec!(300), env)?,
        token_xrd.take(dec!(1000), env)?,
        env,
    )?;
    let remainder = remainder.unwrap();

    // Assert
    assert_eq!(lp_token.amount(env)?, dec!(447.213595499957939282));
    assert_eq!(remainder.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(100));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(400));
    assert_eq!(pool_info.reserve_y, dec!(2000));

    Ok(())
}

#[test]
fn test_remove_liquidity() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let mut pool = vars.fetch::<HyperStake>("pool");

    let (lp_token, _) = pool.add_liquidity(
        token_lsulp.take(dec!(300), env)?,
        token_xrd.take(dec!(1000), env)?,
        env,
    )?;

    // Act
    let (token_lsulp, token_xrd) = pool.remove_liquidity(
        lp_token,
        env,
    )?;
    
    // Assert
    assert_eq!(token_lsulp.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(token_lsulp.amount(env)?, dec!(200));
    assert_eq!(token_xrd.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(token_xrd.amount(env)?, dec!(1000));
    
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(200));
    assert_eq!(pool_info.reserve_y, dec!(1000));

    Ok(())
}

#[test]
fn test_swap_lsulp() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let (token, remainder) = pool.swap(
        token_lsulp.take(dec!(100), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(108.054512372828607566));
    assert_eq!(remainder.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(0));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(200) + dec!(99.8));
    assert_eq!(pool_info.reserve_y, dec!(1000) - dec!(108.054512372828607566));

    Ok(())
}

#[test]
fn test_swap_lsulp_with_remainder() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let (token, remainder) = pool.swap(
        token_lsulp.take(dec!(2000), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(1000));
    assert_eq!(remainder.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(1030.91406097));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(200) + dec!(967.14776717));
    assert_eq!(pool_info.reserve_y, dec!(0));

    Ok(())
}

#[test]
fn test_swap_lsulp_zero_reserve() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let (mut pool, _lp_token) = HyperStake::new_with_tokens(
        token_lsulp.take(dec!(1000), env)?,
        token_xrd.take(dec!(0), env)?,
        dec!(1.1), 
        dec!(0.9), 
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Act
    let (token, remainder) = pool.swap(
        token_lsulp.take(dec!(1000), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(0));
    assert_eq!(remainder.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(1000));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(1000));
    assert_eq!(pool_info.reserve_y, dec!(0));

    Ok(())
}

#[test]
fn test_swap_xrd() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let (token, remainder) = pool.swap(
        token_xrd.take(dec!(100), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(89.76610886));
    assert_eq!(remainder.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(0));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(200) - dec!(89.76610886));
    assert_eq!(pool_info.reserve_y, dec!(1000) + dec!(99.8));

    Ok(())
}

#[test]
fn test_swap_xrd_with_remainder() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let (token, remainder) = pool.swap(
        token_xrd.take(dec!(2000), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(200));
    assert_eq!(remainder.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(1775.830694972562345032));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(0));
    assert_eq!(pool_info.reserve_y, dec!(1000) + dec!(223.72096641738277966));

    Ok(())
}

#[test]
fn test_swap_xrd_zero_reserve() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_lsulp = vars.fetch::<Bucket>("token_lsulp");
    let token_xrd = vars.fetch::<Bucket>("token_xrd");
    let (mut pool, _lp_token) = HyperStake::new_with_tokens(
        token_lsulp.take(dec!(0), env)?,
        token_xrd.take(dec!(1000), env)?,
        dec!(1.1), 
        dec!(0.9), 
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Act
    let (token, remainder) = pool.swap(
        token_xrd.take(dec!(1000), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_lsulp.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(0));
    assert_eq!(remainder.resource_address(env)?, token_xrd.resource_address(env)?);
    assert_eq!(remainder.amount(env)?, dec!(1000));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(0));
    assert_eq!(pool_info.reserve_y, dec!(1000));

    Ok(())
}

#[test]
fn test_swap_z() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_z = vars.fetch::<Bucket>("token_z");
    let mut pool = vars.fetch::<HyperStake>("pool");

    // Act
    let result = pool.swap(
        token_z.take(dec!(100), env)?, 
        env, 
    ); 

    // Assert
    assert!(result.is_err());

    Ok(())
}
