use actix_web::{get, post, web, Error, HttpResponse};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait as _, PaginatorTrait as _, QueryFilter,
    QueryOrder as _,
};

use crate::{
    entities::contract::{Column, Entity},
    primitives::{
        v1::{ContractDeployRequest, ContractFindRequest, ContractResponse},
        DataResponse, ErrorResponse, PaginationRequest, PaginationResponse, Uint256,
    },
    services::ContractManager,
};

#[get("/token")]
pub async fn handle_tokens(
    form: serde_qs::actix::QsQuery<PaginationRequest<ContractFindRequest>>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();

    let mut response = PaginationResponse::<ContractFindRequest, ContractResponse>::builder();

    response.with_size(form.size()).with_page(form.page());

    let db = provider.get_required::<DatabaseConnection>();
    let contract_manager = provider.get_required::<ContractManager>();

    let select = Entity::find();

    let select = match &form.query().chain_id {
        Some(chain_id) => select.filter(Column::ChainId.eq(chain_id)),
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Missing mandatory parameter `chain_id`")
                    .build()
                    .unwrap(),
            ))
        }
    };

    // TODO: filter by type
    // let select = match form.query().r#type {
    //     Some(r#type) => select.filter(Column::Type.is_in(r#type)),
    //     _ => select,
    // };

    let select = match form.query().protocol.clone() {
        Some(protocol) => select.filter(Column::Protocol.is_in(protocol)),
        _ => select,
    };

    let select = select
        .order_by_desc(Column::Id)
        .paginate(db.as_ref(), form.size());

    let total = select.num_pages().await.unwrap_or_default();
    response.with_total(total);

    let contracts = select.fetch_page(form.page()).await.unwrap_or_default();

    for contract in contracts {
        let contract = contract_manager.dump(&contract, true).await.unwrap();

        response.append(contract);
    }

    let response = response.build().unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/token/{contract}")]
pub async fn handle_token(
    path: web::Path<(String,)>,
    form: serde_qs::actix::QsQuery<ContractFindRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let contract_manager = provider.get_required::<ContractManager>();

    let chain_id = match form.chain_id.clone() {
        Some(chain_id) => chain_id,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Missing mandatory parameter `chain_id`")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let contract = match contract_manager.find(&chain_id, &path.0).await {
        Some(contract) => contract,
        None => {
            return Ok(HttpResponse::NotFound().json(
                ErrorResponse::NotFound()
                    .with_error_description("Contract not found")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let response = match contract_manager.dump(&contract, true).await {
        Some(contract) => contract,
        None => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::EmptyImpossible()))
        }
    };

    let response = DataResponse::<ContractResponse>::builder()
        .with_data(response)
        .build()
        .unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/token/{contract}/holder")]
pub async fn handle_holder(
    path: web::Path<(String,)>,
    form: serde_qs::actix::QsQuery<ContractFindRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let contract_manager = provider.get_required::<ContractManager>();

    let chain_id = match form.chain_id.clone() {
        Some(chain_id) => chain_id,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Missing mandatory parameter `chain_id`")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let contract = match contract_manager.find(&chain_id, &path.0).await {
        Some(contract) => contract,
        None => {
            return Ok(HttpResponse::NotFound().json(
                ErrorResponse::NotFound()
                    .with_error_description("Contract not found")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let holders = contract_manager.holder(&contract, 20).await;

    let response = DataResponse::<Vec<(String, i64)>>::builder()
        .with_data(holders)
        .build()
        .unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[post("/token")]
pub async fn handle_token_deploy(
    form: web::Either<
        Result<web::Json<ContractDeployRequest>, Error>,
        Result<web::Form<ContractDeployRequest>, Error>,
    >,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let form = match form {
        web::Either::Left(Ok(form)) => form.into_inner(),
        web::Either::Right(Ok(form)) => form.into_inner(),
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Malformed request")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let contract_manager = provider.get_required::<ContractManager>();

    let chain_id = match form.chain_id.clone() {
        Some(chain_id) => chain_id,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Missing mandatory parameter `chain_id`")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let name = match form.name.clone() {
        Some(name) => name,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Missing mandatory parameter `name`")
                    .build()
                    .unwrap(),
            ))
        }
    };

    if contract_manager.find(&chain_id, &name).await.is_some() {
        return Ok(HttpResponse::BadRequest().json(
            ErrorResponse::Conflict()
                .with_error_description("Token name already taken")
                .build()
                .unwrap(),
        ));
    }

    let mut response = ContractResponse::builder();
    response
        .with_chain_id(&chain_id)
        .with_name(&name)
        .with_symbol(&name);

    if let Some(protocol) = form.protocol {
        response.with_type(protocol).with_protocol(protocol);
    }

    response
        .with_to_address("AeDB27Cc7AEe4Dc74c02CfCc80F71ffF7a3Dfe36")
        .with_fee(Uint256::from_str_prefixed("0x1A055690D9DB80000").unwrap());

    let response = response.build().unwrap();

    let response = DataResponse::<ContractResponse>::builder()
        .with_data(response)
        .build()
        .unwrap();

    Ok(HttpResponse::Accepted().json(response))
}
