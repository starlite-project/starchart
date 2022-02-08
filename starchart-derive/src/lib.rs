#![cfg(not(tarpaulin_include))]
#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::panic_in_result_fn,
	missing_copy_implementations
)]
#![deny(clippy::all)]
#![allow(clippy::module_name_repetitions, clippy::no_effect_underscore_binding)]
//! Derive macro helpers for the starchart crate.

const KEY_IDENT: &str = "key";
const ID_IDENT: &str = "id";

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Field, Fields, Result};

#[proc_macro_derive(IndexEntry, attributes(key))]
pub fn derive_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	parse(&input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn parse(input: &DeriveInput) -> Result<TokenStream> {
	let ident = input.ident.clone();

	let data = match &input.data {
		Data::Struct(st) => st,
		_ => {
			return Err(Error::new_spanned(
				&input,
				"IndexEntry can only be derived on structs",
			))
		}
	};

	let named_fields = match data.fields {
		Fields::Named(ref named) => &named.named,
		_ => {
			return Err(Error::new_spanned(
				&data.fields,
				"IndexEntry can only be derived on a struct with named fields",
			))
		}
	};

	let fields = named_fields.into_iter().cloned().collect::<Vec<_>>();

	let id_field = get_id_field(&fields).ok_or_else(|| {
		Error::new_spanned(
			&input,
			"Expected a #[key] attribute or a field named `key` or `id`.",
		)
	})?;

	let id_ident = id_field
		.ident
		.as_ref()
		.ok_or_else(|| Error::new_spanned(id_field, "expected a named field"))?;

	let id_type = id_field.ty.clone();

	let id_span = id_field.span();

	let implementation = quote_spanned! {id_span=>
		#[automatically_derived]
		impl ::starchart::IndexEntry for #ident {
			type Key = #id_type;

			fn key(&self) -> &Self::Key {
				&self.#id_ident
			}
		}
	};

	let quote_impl = quote! {
		#implementation
	};

	Ok(quote_impl)
}

fn get_id_field(fields: &[Field]) -> Option<&Field> {
	for field in fields {
		if field.attrs.iter().any(|attr| attr.path.is_ident(KEY_IDENT)) {
			return Some(field);
		}
	}

	for field in fields {
		if field
			.ident
			.as_ref()
			.map_or(false, |ident| ident == KEY_IDENT || ident == ID_IDENT)
		{
			return Some(field);
		}
	}

	None
}
