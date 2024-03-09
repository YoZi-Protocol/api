use actix_web::{get, web, Error, HttpResponse};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait as _, PaginatorTrait as _, QueryFilter,
    QueryOrder as _,
};

use crate::{
    entities::{
        extrinsic::{Column, Entity},
        BlockState,
    },
    primitives::{
        v1::{ExtrinsicFindRequest, ExtrinsicResponse},
        DataResponse, ErrorResponse, PaginationRequest, PaginationResponse,
    },
    services::ExtrinsicManager,
};

#[get("/extrinsic")]
pub async fn handle_extrinsics(
    form: serde_qs::actix::QsQuery<PaginationRequest<ExtrinsicFindRequest>>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();

    let mut response = PaginationResponse::<ExtrinsicFindRequest, ExtrinsicResponse>::builder();

    response.with_size(form.size()).with_page(form.page());

    let db = provider.get_required::<DatabaseConnection>();
    let extrinsic_manager = provider.get_required::<ExtrinsicManager>();

    let select = Entity::find();

    let select = select.filter(Column::ChainId.eq(&form.query().chain_id));

    // TODO: filter by block

    let select = match &form.query().tx_hash {
        Some(tx_hash) => select.filter(Column::TxHash.eq(tx_hash)),
        _ => select,
    };

    let select = match &form.query().asset_id {
        Some(assets) => select.filter(Column::AssetId.is_in(assets)),
        _ => select,
    };

    let select = match &form.query().address {
        Some(addresses) => select.filter(
            Column::FromAddress
                .is_in(addresses)
                .or(Column::ToAddress.is_in(addresses)),
        ),
        _ => select,
    };

    let select = select.filter(
        Column::State
            .eq(BlockState::Finalized)
            .or(Column::State.eq(BlockState::Dropped)),
    );

    let select = select
        .order_by_desc(Column::Id)
        .paginate(db.as_ref(), form.size());

    let total = select.num_pages().await.unwrap_or_default();
    response.with_total(total);

    let extrinsics = select.fetch_page(form.page()).await.unwrap_or_default();

    for extrinsic in extrinsics {
        let extrinsic = extrinsic_manager.dump(&extrinsic, true).await.unwrap();

        response.append(extrinsic);
    }

    let response = response.build().unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/extrinsic/{hash}/{index}")]
pub async fn handle_extrinsic(
    path: web::Path<(String, i64)>,
    form: serde_qs::actix::QsQuery<ExtrinsicFindRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let extrinsic_manager = provider.get_required::<ExtrinsicManager>();

    let extrinsic = match extrinsic_manager
        .find(&form.chain_id, &path.0, path.1)
        .await
    {
        Some(extrinsic) => extrinsic,
        None => {
            return Ok(HttpResponse::NotFound().json(
                ErrorResponse::NotFound()
                    .with_error_description("Extrinsic not found")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let response = match extrinsic_manager.dump(&extrinsic, true).await {
        Some(extrinsic) => extrinsic,
        None => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::EmptyImpossible()))
        }
    };

    let response = DataResponse::<ExtrinsicResponse>::builder()
        .with_data(response)
        .build()
        .unwrap();

    Ok(HttpResponse::Ok().json(response))
}
