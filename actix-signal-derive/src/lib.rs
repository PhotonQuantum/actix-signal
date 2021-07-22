extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{parse_macro_input, ItemStruct, Token};

#[proc_macro_derive(SignalHandler)]
pub fn signal_handler_derive(input: TokenStream) -> TokenStream {
    let item: ItemStruct = parse_macro_input!(input);
    let generics = item.generics;
    let name = item.ident;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let maybe_where = where_clause
        .is_none()
        .then(|| Token![where](Span::call_site()));

    let tokens = quote! {
        impl #impl_generics actix::Handler<actix_signal::StopSignal> for #name #ty_generics #where_clause
            #maybe_where Self: actix::Actor
        {
            type Result = ();

            fn handle(&mut self, _msg: actix_signal::StopSignal, ctx: &mut Self::Context) -> Self::Result {
                use actix::ActorContext;
                ctx.stop();
            }
        }
        impl #impl_generics actix::Handler<actix_signal::TerminateSignal> for #name #ty_generics #where_clause
            #maybe_where Self: actix::Actor
        {
            type Result = ();

            fn handle(&mut self, _msg: actix_signal::TerminateSignal, ctx: &mut Self::Context) -> Self::Result {
                use actix::ActorContext;
                ctx.terminate();
            }
        }
    };

    tokens.into()
}
