use scrypto::prelude::*;

#[blueprint]
mod token_creator {
    struct TokenCreator {}

    impl TokenCreator {
        pub fn create_token(
            divisibility: u8,
            initial_supply: Decimal,
            resource_roles: FungibleResourceRoles,
            metadata: ModuleConfig<MetadataInit>,
            with_owner: Option<ModuleConfig<MetadataInit>>,
            reservation: GlobalAddressReservation,
        ) -> (FungibleBucket, Option<FungibleBucket>) {
            let (owner_role, owner_token) = if let Some(owner_metadata) = with_owner {
                let owner_token: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                    .divisibility(0)
                    .burn_roles(burn_roles! {
                        burner => rule!(allow_all);
                        burner_updater => rule!(deny_all);
                    })
                    .metadata(owner_metadata)
                    .mint_initial_supply(dec!(1));

                (
                    OwnerRole::Updatable(rule!(require(owner_token.resource_address()))),
                    Some(owner_token),
                )
            } else {
                (OwnerRole::None, None)
            };

            let FungibleResourceRoles {
                mint_roles,
                burn_roles,
                freeze_roles,
                recall_roles,
                withdraw_roles,
                deposit_roles,
            } = resource_roles;

            let token: FungibleBucket = ResourceBuilder::new_fungible(owner_role)
                .divisibility(divisibility)
                .mint_roles(mint_roles)
                .burn_roles(burn_roles)
                .freeze_roles(freeze_roles)
                .recall_roles(recall_roles)
                .withdraw_roles(withdraw_roles)
                .deposit_roles(deposit_roles)
                .metadata(metadata)
                .with_address(reservation)
                .mint_initial_supply(initial_supply);

            (token, owner_token)
        }
    }
}
