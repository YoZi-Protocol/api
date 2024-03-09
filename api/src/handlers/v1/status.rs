use actix_web::{get, web, Error, HttpResponse};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait as _, PaginatorTrait as _, QueryFilter,
};

use crate::{
    entities::{
        extrinsic::{Column, Entity},
        BlockState,
    },
    primitives::{
        v1::{StatusRequest, StatusResponse},
        ErrorResponse,
    },
};

#[get("/status")]
pub async fn handle_status(
    form: serde_qs::actix::QsQuery<StatusRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let db = provider.get_required::<DatabaseConnection>();

    let select = Entity::find();

    let select = match &form.chain_id {
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

    let select = match &form.asset_id {
        Some(assets) => select.filter(Column::AssetId.is_in(assets)),
        _ => select,
    };

    let pending = select
        .clone()
        .filter(
            Column::State.eq(BlockState::Pending).or(Column::State
                .eq(BlockState::Indexing)
                .or(Column::State.eq(BlockState::Confirmed))),
        )
        .count(db.as_ref())
        .await
        .ok()
        .unwrap_or(0);

    let finalized = select
        .clone()
        .filter(Column::State.eq(BlockState::Finalized))
        .count(db.as_ref())
        .await
        .ok()
        .unwrap_or(0);

    let dropped = select
        .clone()
        .filter(Column::State.eq(BlockState::Dropped))
        .count(db.as_ref())
        .await
        .ok()
        .unwrap_or(0);

    let mut response = StatusResponse::builder();
    response
        .with_pending(pending)
        .with_finalized(finalized)
        .with_dropped(dropped);

    let response = response.build().unwrap();

    Ok(HttpResponse::Ok().json(response))
}
