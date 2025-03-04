pub mod consts;

use scrypto::prelude::*;
pub use consts::*;

#[blueprint]
mod token_creator {
    const LOCKER_COMPONENT: ComponentAddress = _JUST_LOCKER_COMPONENT;
    
    extern_blueprint! {
        WEIGHTED_POOL_PACKAGE,
        WeightedPool {
            fn new_with_tokens(token_x: Bucket, token_y: Bucket, weight_x: Decimal, fee: Decimal, reservation: Option<GlobalAddressReservation>) -> (Global<WeightedPool>, Bucket);
        }
    }

    extern_blueprint! {
        JUST_LOCKER_PACKAGE,
        Locker {
            fn lock(&self, item: Bucket, unlockable_at: Option<Instant>, name: String, description: String, key_image_url: Url) -> Bucket;
        }
    }

    struct TokenCreator {}

    impl TokenCreator {
        pub fn create_token(
            divisibility: u8,
            initial_supply: Decimal,
            resource_roles: FungibleResourceRoles,
            metadata: ModuleConfig<MetadataInit>,
            with_owner: Option<ModuleConfig<MetadataInit>>,
            with_pool: Option<(Decimal, Bucket, Decimal, Decimal, Option<(Option<Instant>, String, String, Url)>)>,
            with_lock_tokens: Option<(Decimal, Option<Instant>, String, String, Url)>,
        ) -> (Bucket, Option<Bucket>, Option<Bucket>, Option<Bucket>, Option<Bucket>) {

            let (owner_role, token_owner) = if let Some(owner_metadata) = with_owner {
                let token_owner: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                    .divisibility(0)
                    .burn_roles(burn_roles! {
                        burner => rule!(allow_all);
                        burner_updater => rule!(deny_all);
                    })
                    .metadata(owner_metadata)
                    .mint_initial_supply(dec!(1))
                    .into();

                (OwnerRole::Updatable(rule!(require(token_owner.resource_address()))), Some(token_owner))
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

            let mut token_x: Bucket = ResourceBuilder::new_fungible(owner_role)
                .divisibility(divisibility)
                .mint_roles(mint_roles)
                .burn_roles(burn_roles)
                .freeze_roles(freeze_roles)
                .recall_roles(recall_roles)
                .withdraw_roles(withdraw_roles)
                .deposit_roles(deposit_roles)
                .metadata(metadata)
                .mint_initial_supply(initial_supply)
                .into();

            let (token_lp, receipt_locker_lp) = if let Some((amount_x, token_y, weight_x, fee, with_lock_lp)) = with_pool {
                let (_, token_lp) = Blueprint::<WeightedPool>::new_with_tokens(
                    token_x.take(amount_x), 
                    token_y, 
                    weight_x, 
                    fee, 
                    None);

                if let Some((unlockable_at, name, description, key_image_url)) = with_lock_lp {
                    let receipt_locker_lp = Global::<Locker>::from(LOCKER_COMPONENT).lock(
                        token_lp,
                        unlockable_at,
                        name,
                        description,
                        key_image_url);
                        
                    (None, Some(receipt_locker_lp))
                } else {
                    (Some(token_lp), None)
                }
            } else {
                (None, None)
            };

            let receipt_locker_token_x = if let Some((amount_x, unlockable_at, name, description, key_image_url)) = with_lock_tokens {
                let receipt_locker_token_x = Global::<Locker>::from(LOCKER_COMPONENT).lock(
                    token_x.take(amount_x),
                    unlockable_at,
                    name,
                    description,
                    key_image_url);

                Some(receipt_locker_token_x)
            } else {
                None
            };

            (token_x, token_owner, token_lp, receipt_locker_lp, receipt_locker_token_x)
        }
    }
}
