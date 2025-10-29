use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Response, StdError, StdResult, WasmMsg, CosmosMsg,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Snip721ExecuteMsg, PublicMetadata, PrivateMetadata, PublicExtension, PrivateExtension, Attribute};
use crate::state::{Config, CONFIG};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner = deps.api.addr_validate(&msg.owner)?;
    let nft_contract = deps.api.addr_validate(&msg.nft_contract)?;

    let config = Config {
        owner,
        nft_contract,
        nft_contract_hash: msg.nft_contract_hash.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", msg.owner)
        .add_attribute("nft_contract", msg.nft_contract)
        .add_attribute("nft_contract_hash", msg.nft_contract_hash))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateConfig { config } => execute_update_config(deps, info, config),
        ExecuteMsg::RegisterScientist {
            name,
            institution,
            specialization,
            description,
            email,
            image,
            website,
            orcid_id,
        } => execute_register_scientist(
            deps,
            env,
            info,
            name,
            institution,
            specialization,
            description,
            email,
            image,
            website,
            orcid_id,
        ),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    config: Config,
) -> StdResult<Response> {
    let old_config = CONFIG.load(deps.storage)?;

    if info.sender != old_config.owner {
        return Err(StdError::generic_err("Unauthorized"));
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn execute_register_scientist(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    institution: String,
    specialization: String,
    description: String,
    email: String,
    image: Option<String>,
    website: Option<String>,
    orcid_id: Option<String>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    // Generate unique token ID with timestamp
    let timestamp = env.block.time.seconds();
    let token_id = format!("scientist-{}", timestamp);

    // Get current time in ISO format
    let registered_at = env.block.time.to_string();

    // Build public metadata
    let public_metadata = PublicMetadata {
        extension: PublicExtension {
            name: format!("{} - Scientist", name),
            description: "Scientist verification token on Secret Network".to_string(),
            image,
            attributes: vec![
                Attribute {
                    trait_type: "Type".to_string(),
                    value: "Scientist".to_string(),
                },
                Attribute {
                    trait_type: "Institution".to_string(),
                    value: institution.clone(),
                },
                Attribute {
                    trait_type: "Specialization".to_string(),
                    value: specialization.clone(),
                },
                Attribute {
                    trait_type: "Network".to_string(),
                    value: "Secret Network Mainnet".to_string(),
                },
                Attribute {
                    trait_type: "Platform".to_string(),
                    value: "Unbound Science".to_string(),
                },
            ],
        },
    };

    // Build private metadata
    let private_metadata = PrivateMetadata {
        extension: PrivateExtension {
            registered_at,
            wallet_address: info.sender.to_string(),
            description: description.clone(),
            platform: "Unbound Science".to_string(),
            email: email.clone(),
            website,
            orcid_id,
        },
    };

    // Create the mint message
    let mint_msg = Snip721ExecuteMsg::MintNft {
        token_id: token_id.clone(),
        owner: info.sender.to_string(),
        public_metadata: Some(public_metadata),
        private_metadata: Some(private_metadata),
        memo: None,
    };

    // Create the WasmMsg to call the NFT contract
    let msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: config.nft_contract.to_string(),
        code_hash: config.nft_contract_hash,
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "register_scientist")
        .add_attribute("token_id", token_id)
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("name", name)
        .add_attribute("institution", institution)
        .add_attribute("specialization", specialization))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
