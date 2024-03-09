use actix_web::{get, web, Error, HttpResponse};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait as _, PaginatorTrait as _, QueryFilter,
    QueryOrder as _,
};

use crate::{
    entities::transaction::{Column, Entity},
    primitives::{
        v1::{TransactionFindRequest, TransactionResponse},
        DataResponse, ErrorResponse, PaginationRequest, PaginationResponse,
    },
    services::TransactionManager,
};

#[get("/transaction")]
pub async fn handle_transactions(
    form: serde_qs::actix::QsQuery<PaginationRequest<TransactionFindRequest>>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();

    let mut response = PaginationResponse::<TransactionFindRequest, TransactionResponse>::builder();

    response.with_size(form.size()).with_page(form.page());

    let db = provider.get_required::<DatabaseConnection>();
    let transaction_manager = provider.get_required::<TransactionManager>();

    let select = Entity::find();

    let select = select.filter(Column::ChainId.eq(&form.query().chain_id));

    let select = match &form.query().block {
        Some(block) => select.filter(Column::BlockHash.eq(block)),
        _ => select,
    };

    // TODO: filter by address[]

    let select = select
        .order_by_desc(Column::Id)
        .paginate(db.as_ref(), form.size());

    let total = select.num_pages().await.unwrap_or_default();
    response.with_total(total);

    let transactions = select.fetch_page(form.page()).await.unwrap_or_default();

    for transaction in transactions {
        let transaction = transaction_manager.dump(&transaction, true).await.unwrap();

        response.append(transaction);
    }

    let response = response.build().unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/transaction/{hash}")]
pub async fn handle_transaction(
    path: web::Path<(String,)>,
    form: serde_qs::actix::QsQuery<TransactionFindRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let transaction_manager = provider.get_required::<TransactionManager>();

    let transaction = match transaction_manager.find(&form.chain_id, &path.0).await {
        Some(transaction) => transaction,
        None => {
            return Ok(HttpResponse::NotFound().json(
                ErrorResponse::NotFound()
                    .with_error_description("Transaction not found")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let response = match transaction_manager.dump(&transaction, true).await {
        Some(transaction) => transaction,
        None => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::EmptyImpossible()))
        }
    };

    let response = DataResponse::<TransactionResponse>::builder()
        .with_data(response)
        .build()
        .unwrap();

    Ok(HttpResponse::Ok().json(response))
}
