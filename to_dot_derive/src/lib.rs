use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_tokenstream;

use quote::quote;
use syn::{
    Ident,
    Field,
    Data, 
    Fields, 
    Attribute,
    DataStruct,
    TypeParam, 
    DeriveInput, 
    FieldsNamed,
    token::Comma, 
    WhereClause,
    ImplGenerics,
    TypeGenerics, 
    spanned::Spanned, 
    parse_macro_input,
    punctuated::Punctuated,
};


mod utils {
    use syn::Error;
    use proc_macro2::{Span, TokenStream as pm2_tokenstream};
    
    pub fn new_err(msg: &str) -> pm2_tokenstream {
        Error::new(Span::call_site(), msg).to_compile_error()
    }

    pub fn new_err_at(span: Span, msg: &str) -> pm2_tokenstream {
        Error::new(span, msg).to_compile_error()
    }
}

#[proc_macro_derive(ToDot, attributes(value, children))]
pub fn to_dot_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match to_dot_derive_impl(&input) {
        Ok(impl_) => impl_.into(),
        Err(err) => err.into()
    }
}

fn to_dot_derive_impl(input: &DeriveInput) -> Result<pm2_tokenstream, pm2_tokenstream> {
    let struct_ident    = input.ident.clone();
    let struct_generics = input.generics.clone();
    let (impl_generics, type_generics, where_clause) = struct_generics.split_for_impl();

    let fields = get_fields(&input.data)?;
    let (value_ident, children_ident) = get_attr_idents(fields)?;

    Ok(impl_dot_trait(
        struct_ident, 
        impl_generics, type_generics, where_clause.cloned(),
        value_ident, children_ident
    ))
}

fn get_fields(data: &Data) -> Result<&Punctuated<Field, Comma>, pm2_tokenstream> 
{
    match data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named) => Ok(&named.named),
                _ => Err(utils::new_err("ToDot can only be derived on structs with named fields"))
            }
        }

        _ => Err(utils::new_err("ToDot can only be derived for structs"))
    }
}


fn get_attr_idents(
    fields: &Punctuated<Field, Comma>
) -> Result<(Ident, Ident), pm2_tokenstream> {
    let mut value_ident = None;
    let mut children_ident = None;

    let get_ident = |field: &Field, ident_str: &str, attr_ident: &Option<Ident>, attr: &Attribute| -> 
    Result<Ident, pm2_tokenstream> {
        if attr_ident.is_some() {
            return Err(
                utils::new_err_at(
                    attr.span(),
                    &format!("Only one {ident_str} is allowed in your struct!")
                )
            );
        }

        field.ident.clone()
            .ok_or_else(|| {
                utils::new_err_at(
                    field.span(),
                    &format!("Expected named field for: #[{ident_str}]")
                )
            })
    }; 

    for field in fields {
        for attr in &field.attrs {
            if attr.path().is_ident("value") {
                value_ident = Some(get_ident(field, "value", &value_ident, attr)?);
            }

            else if attr.path().is_ident("children") {
                // if !is_self_rec(&field.ty, &struct_ident.to_string()) {
                //     return Err(utils::new_err_at(
                //         field.span(),
                //         &format!("#[children] field must be a self-recursive type! e.g: Vec<struct<T>>, [struct<T> ; N]")
                //     ));
                // }

                children_ident = Some(get_ident(field, "children", &children_ident, attr)?);
            }
        }
    }

    Ok((value_ident.unwrap(), children_ident.unwrap()))
}

fn impl_dot_trait(
    struct_ident: Ident,
    impl_generics: ImplGenerics,
    type_generics: TypeGenerics,
    where_clause: Option<WhereClause>,
    value_ident: Ident,
    children_ident: Ident
) -> pm2_tokenstream {
     quote! {
        trait ToDot {
            fn to_dot(&self) -> String;
            fn to_dot_impl(&self, dot: &mut String, root_id: usize, child_id: &mut usize) -> usize;
        }
         
        impl #impl_generics ToDot for #struct_ident #type_generics #where_clause {
            fn to_dot(&self) -> String {
                let mut dot = String::from("digraph G {\n");
                self.to_dot_impl(&mut dot, 0, &mut 0usize);
                dot.push_str("}\n");

                dot
            }
            
            fn to_dot_impl(
                &self,
                dot: &mut String,
                root_id: usize,
                child_id: &mut usize
            ) -> usize {
                dot.push_str(&format!(
                    "\t{} [label=\"{}\"]\n",
                    root_id, self.#value_ident.to_string()
                ));
                
                for child in &self.#children_ident {
                    *child_id += 1;
                    
                    dot.push_str(&format!(
                        "\t{} -> {}\n",
                        root_id, *child_id
                    ));
                    
                    child.to_dot_impl(dot, *child_id, child_id);
                }
                
                root_id
            }
        }
    }
}

// fn is_self_rec(ty: &Type, struct_name: &str) -> bool {
//     match ty {
//         Type::Path(p_ty) => {
//             if let Some(segment) = p_ty.path.segments.last() {
//                 if segment.ident.to_string() == "Vec" {
//                     if let PathArguments::AngleBracketed(args) = &segment.arguments {
//                         for arg in &args.args {
//                             if let GenericArgument::Type(inner_ty) = arg {
//                                 return is_self_rec(inner_ty, struct_name);
//                             } 
//                         }
//                     }
    
//                 }
//             }
//         }

//         Type::Array(a_ty) => is_self_rec(&a_ty.elem, struct_name),

//         _ => false,
//     }
// }

