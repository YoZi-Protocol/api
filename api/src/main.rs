use std::sync::Arc;

use eos420_primitives::{self as primitives, entities};
use eos420_services::{self as services};
use migration::{Migrator, MigratorTrait as _};

use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpResponse, HttpServer,
};

use clap::Parser as _;
use di::Injectable as _;
use sea_orm::{ConnectOptions, Database};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod handlers;

#[cfg(unix)]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    let cli = primitives::Cli::parse();

    let config = match cli.config {
        Some(config) => config::File::from(config),
        None => config::File::with_name("settings").required(false),
    };

    let config = config::Config::builder()
        .add_source(config)
        .add_source(
            config::Environment::with_prefix("EOS420")
                .separator("_")
                .ignore_empty(true)
                .try_parsing(true),
        )
        .build()
        .unwrap();

    let settings = config.try_deserialize::<primitives::Setting>().unwrap();

    env_logger::builder()
        .filter_level(
            settings
                .core()
                .log()
                .parse()
                .unwrap_or(log::LevelFilter::Info),
        )
        .init();

    let mut version = String::new();
    version.push_str(built_info::PKG_VERSION);
    if let Some(commit) = built_info::GIT_COMMIT_HASH_SHORT {
        version.push('+');
        version.push_str(commit);
    }

    log::info!("EOS420/API Version {}", &version);

    log::info!("log level: {}", settings.core().log());

    let mut provider = di::ServiceCollection::new();

    let machine_id = settings.core().machine_id();
    if machine_id == 0xFFFF {
        log::warn!("application is using default machine id");
    }

    provider.add(di::singleton_as_self().from(move |_| {
        let id = services::IdService::new(machine_id);
        Arc::new(id)
    }));

    let mut sea = ConnectOptions::new(settings.database().uri());
    sea.sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(sea).await?;

    Migrator::up(&db, None).await?;

    provider.add(di::singleton_as_self().from(move |_| db.clone().into()));

    provider.add(services::CacheService::<entities::asset::Model>::singleton());
    provider.add(services::CacheService::<entities::block::Model>::singleton());
    provider.add(services::CacheService::<entities::class::Model>::singleton());
    provider.add(services::CacheService::<entities::contract::Model>::singleton());
    provider.add(services::CacheService::<entities::extrinsic::Model>::singleton());
    provider.add(services::CacheService::<entities::transaction::Model>::singleton());

    provider.add(services::AssetManager::scoped());
    provider.add(services::BlockManager::scoped());
    provider.add(services::ClassManager::scoped());
    provider.add(services::ContractManager::scoped());
    provider.add(services::ExtrinsicManager::scoped());
    provider.add(services::LockedAssetManager::scoped());
    provider.add(services::TransactionManager::scoped());

    let provider = provider.build_provider()?;

    log::info!(
        "starting HTTP server at http://{}",
        settings.core().bind().api()
    );

    let server = HttpServer::new(move || {
        let qs_config = serde_qs::Config::new(2, false);
        let qs_query_config = serde_qs::actix::QsQueryConfig::default()
            .error_handler(|err, _| {
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(
                        primitives::ErrorResponse::InvalidRequest()
                            .with_error_description("Malformed query string")
                            .build()
                            .unwrap(),
                    ),
                )
                .into()
            })
            .qs_config(qs_config);

        let form_config = web::FormConfig::default().error_handler(|err, _| {
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(
                    primitives::ErrorResponse::InvalidRequest()
                        .with_error_description("Malformed JSON body")
                        .build()
                        .unwrap(),
                ),
            )
            .into()
        });

        let json_config = web::JsonConfig::default().error_handler(|err, _| {
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(
                    primitives::ErrorResponse::InvalidRequest()
                        .with_error_description("Malformed JSON body")
                        .build()
                        .unwrap(),
                ),
            )
            .into()
        });

        App::new()
            .app_data(qs_query_config)
            .app_data(form_config)
            .app_data(json_config)
            .app_data(web::Data::new(provider.clone()))
            .service(
                web::scope("/api/v1")
                    .service(handlers::v1::handle_status)
                    .service(handlers::v1::handle_tokens)
                    .service(handlers::v1::handle_token)
                    .service(handlers::v1::handle_holder)
                    .service(handlers::v1::handle_token_deploy)
                    .service(handlers::v1::handle_transactions)
                    .service(handlers::v1::handle_transaction)
                    .service(handlers::v1::handle_extrinsics)
                    .service(handlers::v1::handle_extrinsic)
                    .service(handlers::v1::handle_assets)
                    .service(handlers::v1::handle_asset)
                    .service(handlers::v1::handle_nonfungible),
            )
            .default_service(web::to(HttpResponse::NotFound))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(NormalizePath::trim())
    })
    .workers(settings.core().workers())
    .worker_max_blocking_threads(settings.core().blocking_threads())
    .bind(settings.core().bind().api())?
    .run();

    server.await?;

    Ok(())
}
