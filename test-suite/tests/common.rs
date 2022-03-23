use starchart::{Action, Backend, Entry, Error as ChartError, Result, Starchart};

pub async fn setup_gateway<B: Backend, E: Entry>(backend: B, table: &str) -> Result<Starchart<B>> {
	let chart = Starchart::new(backend)
		.await
		.map_err(|e| ChartError::backend(Some(Box::new(e))))?;

	let action: Action<E> = Action::new(table);

	action.create_table(&chart).await?;

	Ok(chart)
}
