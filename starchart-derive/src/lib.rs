#![cfg(not(tarpaulin_include))]
#![feature(never_type, doc_cfg)]
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
#![allow(
	clippy::module_name_repetitions,
	clippy::no_effect_underscore_binding,
	dead_code,
	warnings
)]
//! Derive macro helpers for the starchart crate.

const KEY_IDENT: &str = "key";
const ID_IDENT: &str = "id";

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Error, Field, Fields, Ident,
	Result, Type,
};

#[proc_macro_derive(Key, attributes(key))]
pub fn derive_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	parse(input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn parse(input: DeriveInput) -> Result<TokenStream> {
	let ident = &input.ident;

	let data = match input.data {
		Data::Struct(ref st) => st,
		_ => {
			return Err(Error::new_spanned(
				input,
				"Key can only be derived on structs",
			))
		}
	};

	let named_fields = match data.fields {
		Fields::Named(ref named) => &named.named,
		_ => {
			return Err(Error::new_spanned(
				&data.fields,
				"Key can only be derived on a struct with named fields",
			))
		}
	};

	let fields = named_fields.clone().into_iter().collect::<Vec<_>>();

	let id_field = match get_id_field(&fields) {
		Some(field) => field,
		None => {
			return Err(Error::new_spanned(
				input,
				"Expected a #[key] attribute or a field named `key` or `id`.",
			))
		}
	};

	let id_ident = match &id_field.ident {
		None => return Err(Error::new_spanned(id_field, "expected a named field")),
		Some(f) => f,
	};

	let id_type = &id_field.ty;

	let id_span = id_field.span();

	let implementation = quote_spanned! {id_span=>
		#[automatically_derived]
		impl ::starchart::Key for #ident {
			fn to_key(&self) -> std::string::String {
				<#id_type as ::std::string::ToString>::to_string(&self.#id_ident)
			}
		}
	};

	Ok(implementation)
}

fn get_id_field(fields: &[Field]) -> Option<&Field> {
	for field in fields {
		let attrs = &field.attrs;
		if attrs.is_empty() {
			continue;
		}

		for attr in attrs {
			if attr.path.is_ident(KEY_IDENT) {
				return Some(field);
			}
		}
	}

	let span = Span::call_site();
	for field in fields {
		let ident = match &field.ident {
			None => continue,
			Some(i) => i,
		};
		if ident == KEY_IDENT || ident == ID_IDENT {
			return Some(field);
		}
	}

	None
}
