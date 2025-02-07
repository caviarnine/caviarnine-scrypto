mod common;

use common::*;

#[test]
fn test_new() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_x = vars.fetch::<Bucket>("token_x");
    let token_y = vars.fetch::<Bucket>("token_y");

    // Act
    let pool = WeightedPool::new(
        token_x.resource_address(env)?,
        token_y.resource_address(env)?,
        dec!(0.9), 
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(0));
    assert_eq!(pool_info.resource_x, token_x.resource_address(env)?);
    assert_eq!(pool_info.resource_y, token_y.resource_address(env)?);
    assert_eq!(pool_info.reserve_x, dec!(0));
    assert_eq!(pool_info.reserve_y, dec!(0));
    assert_eq!(pool_info.weight_x, dec!(0.9));
    assert_eq!(pool_info.weight_y, dec!(0.1));
    assert_eq!(pool_info.fee, dec!(0.001));
    assert_eq!(pool_info.protocol_fee_share, dec!(0.1));
    assert_eq!(pool_info.treasury_fee_share, dec!(0.1));

    Ok(())
}

#[test]
fn test_new_with_tokens() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_x = vars.fetch::<Bucket>("token_x");
    let token_y = vars.fetch::<Bucket>("token_y");

    // Act
    let (pool, lp_token) = WeightedPool::new_with_tokens(
        token_x.take(dec!(9000), env)?,
        token_y.take(dec!(1000), env)?,
        dec!(0.9), 
        dec!(0.001), 
        None,
        vars.fetch::<PackageAddress>("pool_package"), 
        env, 
    )?;

    // Assert
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.price, dec!(1));
    assert_eq!(pool_info.resource_x, token_x.resource_address(env)?);
    assert_eq!(pool_info.resource_y, token_y.resource_address(env)?);
    assert_eq!(pool_info.reserve_x, dec!(9000));
    assert_eq!(pool_info.reserve_y, dec!(1000));
    assert_eq!(pool_info.weight_x, dec!(0.9));
    assert_eq!(pool_info.weight_y, dec!(0.1));
    assert_eq!(pool_info.fee, dec!(0.001));
    assert_eq!(pool_info.protocol_fee_share, dec!(0.1));
    assert_eq!(pool_info.treasury_fee_share, dec!(0.1));
    assert_eq!(pool_info.lp_resource, lp_token.resource_address(env)?);

    Ok(())
}

#[test]
fn test_set_protocol_fee_share() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let mut pool = vars.fetch::<WeightedPool>("pool");

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
    let mut pool = vars.fetch::<WeightedPool>("pool");

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
    let mut pool = vars.fetch::<WeightedPool>("pool");

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
    let mut pool = vars.fetch::<WeightedPool>("pool");

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
    let mut pool = vars.fetch::<WeightedPool>("pool");

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
    let mut pool = vars.fetch::<WeightedPool>("pool");

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
    let token_x = vars.fetch::<Bucket>("token_x");
    let token_y = vars.fetch::<Bucket>("token_y");
    let mut pool = vars.fetch::<WeightedPool>("pool");

    // Act
    let (lp_token, remainder) = pool.add_liquidity(
        token_x.take(dec!(1100), env)?,
        token_y.take(dec!(1000), env)?,
        env,
    )?;

    // Assert
    assert_eq!(lp_token.amount(env)?, dec!(1000));
    assert_eq!(remainder.unwrap().amount(env)?, dec!(100));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(2000));
    assert_eq!(pool_info.reserve_y, dec!(2000));

    Ok(())
}

#[test]
fn test_remove_liquidity() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_x = vars.fetch::<Bucket>("token_x");
    let token_y = vars.fetch::<Bucket>("token_y");
    let mut pool = vars.fetch::<WeightedPool>("pool");

    let (lp_token, _) = pool.add_liquidity(
        token_x.take(dec!(1000), env)?,
        token_y.take(dec!(1000), env)?,
        env,
    )?;

    // Act
    let (token_x, token_y) = pool.remove_liquidity(
        lp_token,
        env,
    )?;
    
    // Assert
    assert_eq!(token_x.amount(env)?, dec!(1000));
    assert_eq!(token_y.amount(env)?, dec!(1000));
    
    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(1000));
    assert_eq!(pool_info.reserve_y, dec!(1000));

    Ok(())
}

#[test]
fn test_swap_x() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_x = vars.fetch::<Bucket>("token_x");
    let token_y = vars.fetch::<Bucket>("token_y");
    let mut pool = vars.fetch::<WeightedPool>("pool");

    // Act
    let token = pool.swap(
        token_x.take(dec!(100), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_y.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(90.081892));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(1000) + dec!(99.8));
    assert_eq!(pool_info.reserve_y, dec!(1000) - dec!(90.081892));

    Ok(())
}

#[test]
fn test_swap_y() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_x = vars.fetch::<Bucket>("token_x");
    let token_y = vars.fetch::<Bucket>("token_y");
    let mut pool = vars.fetch::<WeightedPool>("pool");

    // Act
    let token = pool.swap(
        token_y.take(dec!(100), env)?, 
        env, 
    )?;

    // Assert
    assert_eq!(token.resource_address(env)?, token_x.resource_address(env)?);
    assert_eq!(token.amount(env)?, dec!(90.08189262));

    let pool_info = pool.get_info(env)?;
    assert_eq!(pool_info.reserve_x, dec!(1000) - dec!(90.08189262));
    assert_eq!(pool_info.reserve_y, dec!(1000) + dec!(99.8));

    Ok(())
}

#[test]
fn test_swap_z() -> Result<(), RuntimeError> {
    // Arrange
    let (vars, env) = &mut setup()?;
    let token_z = vars.fetch::<Bucket>("token_z");
    let mut pool = vars.fetch::<WeightedPool>("pool");

    // Act
    let result = pool.swap(
        token_z.take(dec!(100), env)?, 
        env, 
    ); 

    // Assert
    assert!(result.is_err());

    Ok(())
}
