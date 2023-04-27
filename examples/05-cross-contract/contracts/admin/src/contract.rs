use crate::error::ContractError;
use crate::msg::{AdminsListResp, ExecuteMsg, InstantiateMsg, JoinTimeResp, QueryMsg};
use crate::state::{ADMINS, DONATION_DENOM};
use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    for addr in msg.admins {
        let admin = deps.api.addr_validate(&addr)?;
        ADMINS.save(deps.storage, &admin, &env.block.time)?;
    }
    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        AdminsList {} => to_binary(&query::admins_list(deps)?),
        JoinTime { admin } => to_binary(&query::join_time(deps, admin)?),
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Leave {} => exec::leave(deps, info).map_err(Into::into),
        Donate {} => exec::donate(deps, info),
    }
}

mod exec {
    use super::*;

    pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        ADMINS.remove(deps.storage, &info.sender);

        let resp = Response::new()
            .add_attribute("action", "leave")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }

    pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let denom = DONATION_DENOM.load(deps.storage)?;
        let admins: Result<Vec<_>, _> = ADMINS
            .keys(deps.storage, None, None, Order::Ascending)
            .collect();
        let admins = admins?;

        let donation = cw_utils::must_pay(&info, &denom)?.u128();

        let donation_per_admin = donation / (admins.len() as u128);

        let messages = admins.into_iter().map(|admin| BankMsg::Send {
            to_address: admin.to_string(),
            amount: coins(donation_per_admin, &denom),
        });

        let resp = Response::new()
            .add_messages(messages)
            .add_attribute("action", "donate")
            .add_attribute("amount", donation.to_string())
            .add_attribute("per_admin", donation_per_admin.to_string());

        Ok(resp)
    }
}

mod query {
    use super::*;

    pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
        let admins: Result<Vec<_>, _> = ADMINS
            .keys(deps.storage, None, None, Order::Ascending)
            .collect();
        let admins = admins?;
        let resp = AdminsListResp { admins };
        Ok(resp)
    }

    pub fn join_time(deps: Deps, admin: String) -> StdResult<JoinTimeResp> {
        ADMINS
            .load(deps.storage, &Addr::unchecked(admin))
            .map(|joined| JoinTimeResp { joined })
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::msg::AdminsListResp;

    use super::*;

    #[test]
    fn instantiation() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec![],
                    donation_denom: "eth".to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(resp, AdminsListResp { admins: vec![] });

        let block = app.block_info();
        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                    donation_denom: "eth".to_owned(),
                },
                &[],
                "Contract 2",
                None,
            )
            .unwrap();

        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(
            resp,
            AdminsListResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );

        let resp: JoinTimeResp = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::JoinTime {
                    admin: "admin1".to_owned(),
                },
            )
            .unwrap();
        assert_eq!(resp.joined, block.time);

        let resp: JoinTimeResp = app
            .wrap()
            .query_wasm_smart(
                addr.clone(),
                &QueryMsg::JoinTime {
                    admin: "admin2".to_owned(),
                },
            )
            .unwrap();
        assert_eq!(resp.joined, block.time);

        app.wrap()
            .query_wasm_smart::<JoinTimeResp>(
                addr,
                &QueryMsg::JoinTime {
                    admin: "admin3".to_owned(),
                },
            )
            .unwrap_err();
    }

    #[test]
    fn donations() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked("user"), coins(5, "eth"))
                .unwrap()
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                    donation_denom: "eth".to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::Donate {},
            &coins(5, "eth"),
        )
        .unwrap();

        assert_eq!(
            app.wrap()
                .query_balance("user", "eth")
                .unwrap()
                .amount
                .u128(),
            0
        );

        assert_eq!(
            app.wrap()
                .query_balance(&addr, "eth")
                .unwrap()
                .amount
                .u128(),
            1
        );

        assert_eq!(
            app.wrap()
                .query_balance("admin1", "eth")
                .unwrap()
                .amount
                .u128(),
            2
        );

        assert_eq!(
            app.wrap()
                .query_balance("admin2", "eth")
                .unwrap()
                .amount
                .u128(),
            2
        );
    }
}
