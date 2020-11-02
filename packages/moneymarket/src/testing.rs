use crate::mock_querier::mock_dependencies;
use crate::querier::{
    compute_tax, deduct_tax, load_all_balances, load_balance, load_distribution_params,
    load_supply, load_token_balance, DistributionParamsResponse,
};
use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cosmwasm_std::{Coin, Decimal, HumanAddr, Uint128};

#[test]
fn token_balance_querier() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_token_balances(&[(
        &HumanAddr::from("liquidity0000"),
        &[(&HumanAddr::from(MOCK_CONTRACT_ADDR), &Uint128(123u128))],
    )]);

    assert_eq!(
        Uint128(123u128),
        load_token_balance(
            &deps,
            &HumanAddr::from("liquidity0000"),
            &HumanAddr::from(MOCK_CONTRACT_ADDR),
        )
        .unwrap()
    );
}

#[test]
fn balance_querier() {
    let deps = mock_dependencies(
        20,
        &[Coin {
            denom: "uusd".to_string(),
            amount: Uint128(200u128),
        }],
    );

    assert_eq!(
        load_balance(
            &deps,
            &HumanAddr::from(MOCK_CONTRACT_ADDR),
            "uusd".to_string()
        )
        .unwrap(),
        Uint128(200u128)
    );
}

#[test]
fn all_balances_querier() {
    let deps = mock_dependencies(
        20,
        &[
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128(200u128),
            },
            Coin {
                denom: "ukrw".to_string(),
                amount: Uint128(100u128),
            },
        ],
    );

    assert_eq!(
        load_all_balances(&deps, &HumanAddr::from(MOCK_CONTRACT_ADDR),).unwrap(),
        vec![
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128(200u128),
            },
            Coin {
                denom: "ukrw".to_string(),
                amount: Uint128(100u128),
            }
        ]
    );
}

#[test]
fn a_value_querier() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_distribution_params(&[(
        &HumanAddr::from("asset0000"),
        &(Decimal::percent(70), Decimal::percent(1)),
    )]);

    assert_eq!(
        load_distribution_params(
            &deps,
            &HumanAddr::from("overseer"),
            &HumanAddr::from("asset0000")
        )
        .unwrap(),
        DistributionParamsResponse {
            a_value: Decimal::percent(70),
            buffer_tax_rate: Decimal::percent(1),
        }
    );
}

#[test]
fn supply_querier() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_token_balances(&[(
        &HumanAddr::from("liquidity0000"),
        &[
            (&HumanAddr::from(MOCK_CONTRACT_ADDR), &Uint128(123u128)),
            (&HumanAddr::from("addr00000"), &Uint128(123u128)),
            (&HumanAddr::from("addr00001"), &Uint128(123u128)),
            (&HumanAddr::from("addr00002"), &Uint128(123u128)),
        ],
    )]);

    assert_eq!(
        load_supply(&deps, &HumanAddr::from("liquidity0000")).unwrap(),
        Uint128(492u128)
    )
}

#[test]
fn test_compute_tax() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_tax(
        Decimal::percent(1),
        &[(&"uusd".to_string(), &Uint128::from(1000000u128))],
    );

    // cap to 1000000
    assert_eq!(
        compute_tax(&deps, &Coin::new(10000000000u128, "uusd")).unwrap(),
        Uint128(1000000u128)
    );

    // normal tax
    assert_eq!(
        compute_tax(&deps, &Coin::new(50000000u128, "uusd")).unwrap(),
        Uint128(495050u128)
    );
}

#[test]
fn test_deduct_tax() {
    let mut deps = mock_dependencies(20, &[]);

    deps.querier.with_tax(
        Decimal::percent(1),
        &[(&"uusd".to_string(), &Uint128::from(1000000u128))],
    );

    // cap to 1000000
    assert_eq!(
        deduct_tax(&deps, Coin::new(10000000000u128, "uusd")).unwrap(),
        Coin {
            denom: "uusd".to_string(),
            amount: Uint128(9999000000u128)
        }
    );

    // normal tax
    assert_eq!(
        deduct_tax(&deps, Coin::new(50000000u128, "uusd")).unwrap(),
        Coin {
            denom: "uusd".to_string(),
            amount: Uint128(49504950u128)
        }
    );
}