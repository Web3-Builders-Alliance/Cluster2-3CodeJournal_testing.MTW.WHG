#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Addr, Order};
//use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{MessagesResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CURRENT_ID, MESSAGES,Message};

// version info for migration info
//const CONTRACT_NAME: &str = "crates.io:messages";
//const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CURRENT_ID.save(deps.storage, &Uint128::zero().u128())?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddMessage { topic, message } => add_message(deps, info, topic, message),
    }
}

pub fn add_message(deps: DepsMut, info:MessageInfo, topic:String, message:String) -> Result<Response, ContractError> {
    //load current id
    let mut current_id = CURRENT_ID.load(deps.storage)?;

    //create new message
    let new_message = Message {
        id: Uint128::from(current_id),
        owner: info.sender,
        topic: topic,
        message: message
    };

    //increment current id
    current_id = current_id.checked_add(1).unwrap();

    MESSAGES.save(deps.storage, new_message.id.u128(), &new_message)?;


    //save current id
    CURRENT_ID.save(deps.storage, &current_id)?;
    
    Ok(Response::new()
        .add_attribute("action", "add_message")
        .add_attribute("id", new_message.id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCurrentId {  } => to_binary(&query_current_id(deps)?),
        QueryMsg::GetAllMessage {} => to_binary(&query_all_messages(deps)?),
        QueryMsg::GetMessagesByAddr { address } => to_binary(&query_messages_by_addr(deps, address)?),
        QueryMsg::GetMessagesByTopic { topic } => to_binary(&query_messages_by_topic(deps, topic)?),
        QueryMsg::GetMessagesById { id } => to_binary(&query_messages_by_id(deps, id)?),
    }
}

fn query_current_id(deps: Deps) -> StdResult<Uint128> {
    let current_id = CURRENT_ID.load(deps.storage)?;
    Ok(Uint128::from(current_id))
}

fn query_all_messages(deps: Deps) -> StdResult<MessagesResponse> {
    let messages: Vec<Message> = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.unwrap().1)
        .collect();
    Ok(MessagesResponse { messages })
}

fn query_messages_by_addr(deps: Deps, address: String) -> StdResult<MessagesResponse> {
    let messages: Vec<Message> = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.unwrap().1)
        .filter(|message| message.owner == address)
        .collect();
    Ok(MessagesResponse { messages })
}

fn query_messages_by_topic(deps: Deps, topic: String) -> StdResult<MessagesResponse> {
    let messages: Vec<Message> = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.unwrap().1)
        .filter(|message| message.topic == topic)
        .collect();
    Ok(MessagesResponse { messages })
}

fn query_messages_by_id(deps: Deps, id: Uint128) -> StdResult<MessagesResponse> {
    let message = MESSAGES.load(deps.storage, id.u128())?;
    Ok(MessagesResponse { messages: vec![message] })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    const SENDER: &str = "sender_address";
    const SENDER2: &str = "sender_address2";

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg { };
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    fn add_message(deps: DepsMut, sender:&str, topic: String, message: String) {
        let msg = ExecuteMsg::AddMessage {
            topic: topic,
            message: message
        };
        let info = mock_info(sender, &[]);
        execute(deps, mock_env(), info, msg).unwrap();
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentId {}).unwrap();
        let value: Uint128 = from_binary(&res).unwrap();
        assert_eq!(Uint128::zero(), value);
    }

    #[test]
    fn add_2_messages_and_query_all_messages() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        add_message(deps.as_mut(), SENDER, "topic".to_string(), "message1".to_string());
        add_message(deps.as_mut(), SENDER,"topic".to_string(), "message2".to_string());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAllMessage {}).unwrap();
        let value: MessagesResponse = from_binary(&res).unwrap();
        assert_eq!(2, value.messages.len());
    }

    #[test]
    fn add_messages_from_two_owners_and_query_messages_by_owner() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        add_message(deps.as_mut(), SENDER, "topic".to_string(), "message1".to_string());
        add_message(deps.as_mut(), SENDER,"topic".to_string(), "message2".to_string());
        add_message(deps.as_mut(), SENDER2, "topic".to_string(), "message3".to_string());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessagesByAddr { address: SENDER.to_string() }).unwrap();
        let value: MessagesResponse = from_binary(&res).unwrap();
        assert_eq!(2, value.messages.len());
    }

    #[test]
    fn add_2_messages_and_query_messages_by_id() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        add_message(deps.as_mut(), SENDER, "topic".to_string(), "message1".to_string());
        add_message(deps.as_mut(), SENDER,"topic".to_string(), "message2".to_string());
        add_message(deps.as_mut(), SENDER2, "topic".to_string(), "message3".to_string());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessagesById { id: Uint128::from(1u64) }).unwrap();
        let value: MessagesResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.messages.len());
    }

    #[test]
    fn query_messages_by_topic() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        add_message(deps.as_mut(), SENDER, "topic".to_string(), "message1".to_string());
        add_message(deps.as_mut(), SENDER,"topic".to_string(), "message2".to_string());
        add_message(deps.as_mut(), SENDER2, "topic".to_string(), "message3".to_string());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessagesByTopic { topic: "topic".to_string() }).unwrap();
        let value: MessagesResponse = from_binary(&res).unwrap();
        assert_eq!(3, value.messages.len());

    }
}
