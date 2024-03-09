use actix_web::{get, web, Error, HttpResponse};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait as _, PaginatorTrait as _, QueryFilter,
    QueryOrder as _,
};

use crate::{
    entities::asset::{Column, Entity},
    primitives::{
        v1::{AssetFindRequest, AssetResponse},
        DataResponse, ErrorResponse, PaginationRequest, PaginationResponse,
    },
    services::AssetManager,
};

#[get("/asset")]
pub async fn handle_assets(
    form: serde_qs::actix::QsQuery<PaginationRequest<AssetFindRequest>>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();

    let mut response = PaginationResponse::<AssetFindRequest, AssetResponse>::builder();

    response.with_size(form.size()).with_page(form.page());

    let db = provider.get_required::<DatabaseConnection>();
    let asset_manager = provider.get_required::<AssetManager>();

    let select = Entity::find();

    let select = match &form.query().chain_id {
        Some(chain_id) => select.filter(Column::ChainId.eq(chain_id)),
        _ => select,
    };

    let select = if let Some(address) = &form.query().address {
        select.filter(Column::Address.eq(address))
    } else if let Some(assets) = &form.query().asset_id {
        select.filter(Column::AssetId.is_in(assets))
    } else {
        return Ok(HttpResponse::BadRequest().json(
            ErrorResponse::InvalidRequest()
                .with_error_description("Missing mandatory parameter `address`")
                .build()
                .unwrap(),
        ));
    };

    let select = select
        .order_by_desc(Column::Id)
        .paginate(db.as_ref(), form.size());

    let total = select.num_pages().await.unwrap_or_default();
    response.with_total(total);

    let assets = select.fetch_page(form.page()).await.unwrap_or_default();

    for asset in assets {
        let asset = asset_manager.dump(&asset, true).await.unwrap();

        response.append(asset);
    }

    let response = response.build().unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/asset/{asset}")]
pub async fn handle_asset(
    path: web::Path<(String,)>,
    form: serde_qs::actix::QsQuery<AssetFindRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let asset_manager = provider.get_required::<AssetManager>();

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

    let address = match form.address.clone() {
        Some(address) => address,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                ErrorResponse::InvalidRequest()
                    .with_error_description("Missing mandatory parameter `address`")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let asset = match asset_manager.find(&chain_id, &path.0, &address).await {
        Some(asset) => asset,
        None => {
            return Ok(HttpResponse::NotFound().json(
                ErrorResponse::NotFound()
                    .with_error_description("Asset not found")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let response = match asset_manager.dump(&asset, true).await {
        Some(asset) => asset,
        None => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::EmptyImpossible()))
        }
    };

    let response = DataResponse::<AssetResponse>::builder()
        .with_data(response)
        .build()
        .unwrap();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/asset/{asset}/{index}")]
pub async fn handle_nonfungible(
    path: web::Path<(String, String)>,
    form: serde_qs::actix::QsQuery<AssetFindRequest>,
    provider: web::Data<di::ServiceProvider>,
) -> Result<HttpResponse, Error> {
    let asset_manager = provider.get_required::<AssetManager>();

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

    let asset = match asset_manager.find_single(&chain_id, &path.0, &path.1).await {
        Some(asset) => asset,
        None => {
            return Ok(HttpResponse::NotFound().json(
                ErrorResponse::NotFound()
                    .with_error_description("Asset not found")
                    .build()
                    .unwrap(),
            ))
        }
    };

    let response = match asset_manager.dump(&asset, true).await {
        Some(asset) => asset,
        None => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse::EmptyImpossible()))
        }
    };

    let response = DataResponse::<AssetResponse>::builder()
        .with_data(response)
        .build()
        .unwrap();

    Ok(HttpResponse::Ok().json(response))
}
