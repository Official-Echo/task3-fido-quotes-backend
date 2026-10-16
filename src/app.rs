use loco_rs::{
	app::{AppContext, Hooks, Initializer},
	bgworker::Queue,
	boot::{create_app, BootResult, StartMode},
	config::Config,
	controller::AppRoutes,
	db::{self, truncate_table},
	environment::Environment,
	prelude::async_trait,
	task::Tasks,
	Result,
};
use migration::Migrator;
use std::path::Path;

use crate::models::quotes::Entity;
#[allow(unused_imports)]
use crate::{controllers /* models::_entities::users */};

pub struct App;
#[async_trait]
impl Hooks for App {
	fn app_name() -> &'static str {
		env!("CARGO_CRATE_NAME")
	}

	fn app_version() -> String {
		format!(
			"{} ({})",
			env!("CARGO_PKG_VERSION"),
			option_env!("BUILD_SHA")
				.or(option_env!("GITHUB_SHA"))
				.unwrap_or("dev")
		)
	}

	async fn boot(
		mode: StartMode,
		environment: &Environment,
		config: Config,
	) -> Result<BootResult> {
		create_app::<Self, Migrator>(mode, environment, config).await
	}

	async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
		Ok(vec![])
	}

	fn routes(_ctx: &AppContext) -> AppRoutes {
		AppRoutes::with_default_routes() // controller routes below
			.add_route(controllers::quotes::routes())
	}
	async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
		Ok(())
	}

	#[allow(unused_variables)]
	fn register_tasks(tasks: &mut Tasks) {
		// tasks-inject (do not remove)
	}
	async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, Entity).await?;
        Ok(())
    }
	async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
		Ok(())
	}
}
