// use crate::attr::*;
use crate::syn::spanned::Spanned;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;

use syn::{
    FnArg, Ident, LitInt, Pat, PatIdent, ReturnType, Type,
    TypePath,
};

macro_rules! data_types {
    ( $( $type:ident ( $( $tystr:tt )* ) , )* ) => {
        #[derive(Debug)]
        enum DataType {
            $( $type (Ident), )*
        }
        impl DataType {
            pub fn name_token(&self) -> Ident {
                match self {
                    $( DataType:: $type (s) => s.clone(), )*
                }
            }

            pub fn parse_str(ty: &str, ident: Ident) -> Option<DataType> {
                match ty {
                    $(
                        $( $tystr )* => Some( DataType:: $type (ident) ),
                    )*
                    _ => None,
                }.into()
            }

            pub fn type_token(&self) -> TokenStream2 {
                match self {
                    $(
                        DataType:: $type (_) => quote! {
                            Document :: $type
                        },
                    )*
                }.into()
            }
        }
    };
}

data_types! {
    String  ("String" | "&str"),
    Char    ("char"),
    F64     ("f64"),
    F32     ("f32"),
    I128    ("i128" | "isize"),
    I64     ("i64"),
    I32     ("i32"),
    I16     ("i16"),
    I8      ("i8"),
    U128    ("u128" | "usize"),
    U64     ("u64"),
    U32     ("u32"),
    U16     ("u16"),
    U8      ("u8"),
}

macro_rules! fail {
    (=> $span:expr , $msg:literal) => {
        Err((quote_spanned! {
            $span => compile_error!($msg);
        })
        .into())
    };
    ($span:expr , $msg:literal) => {
        (quote_spanned! {
            $span => compile_error!($msg);
        })
        .into()
    };
}

fn get_types_from_args(args: &[FnArg]) -> Result<Vec<DataType>, TokenStream2> {
    let mut result = vec![];
    for fn_arg in args.iter() {
        let (arg_id, name) = match fn_arg {
            FnArg::Typed(p) => match (&*p.ty, &*p.pat) {
                (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) => {
                    (path.get_ident(), ident.clone())
                }
                _ => return fail!(=> p.ty.span(), "Unsupport argument type"),
            },
            _ => {
                return fail!(=> fn_arg.span(), "This macro does not support functions with 'self' as a receiver")
            }
        };
        let id_string = match arg_id {
            Some(id) => id.to_string(),
            None => return fail!(=> fn_arg.span(), "Could not get ident from argument"),
        };
        let item = DataType::parse_str(id_string.as_str(), name);
        match item {
            Some(item) => result.push(item),
            None => return fail!(=> fn_arg.span(), "Unsupported argument"),
        }
    }
    Ok(result)
}

pub fn impl_filter(item_fn: &syn::ItemFn) -> TokenStream {
    let mut orig_fn = item_fn.clone();

    //function name
    let ident = &item_fn.sig.ident;
    let inner_fn = format!("__{}_inner__", ident);
    orig_fn.sig.ident = Ident::new(&inner_fn, ident.span());
    let inner_fn_ident = &orig_fn.sig.ident;
    // pub or not pub
    let vis = &item_fn.vis;
    // The arguments to the function
    let mut args: Vec<FnArg> = item_fn.sig.inputs.iter().cloned().collect();

    // Return type
    let return_ty = match &item_fn.sig.output {
        ReturnType::Type(_, ty) => ty,
        _ => return fail!(item_fn.sig.output.span(), "Return type is required"),
    };

    // string form of return type, only represents the last segment of the path. e.g. std::string::String would be just "String"
    let return_ty_string = match &**return_ty {
        Type::Path(TypePath { path, .. }) => path.segments.iter().last().unwrap().ident.to_string(),
        _ => return fail!(return_ty.span(), "This return type is not supported!"),
    };

    if args.is_empty() {
        return fail!(
            ident.span(),
            "Filters require at least one argument for the incoming filtered value"
        );
    }

    let mut data_types = match get_types_from_args(&args) {
        Ok(val) => val,
        Err(e) => return e.into(),
    };

    let filter_in = args.remove(0);
    let filter_in_dt = data_types.remove(0);
    let filter_in_type = filter_in_dt.type_token();
    let filter_in_name = filter_in_dt.name_token();

    let arg_count: LitInt = LitInt::new(&args.len().to_string(), item_fn.sig.ident.span());

    let arg_condition = if args.len() > 1 {
        quote!{
            let mut filter_args: Vec<Document> = match filter_args.into_result() {
                Ok(Document::Seq(val)) => val,
                Err(e) => return e.into(),
                _ => return TemplarError::RenderFailure("Unexpected execution, arguments must be a sequence".to_string()).into(),
            };

            if filter_args.len() != #arg_count {
                return TemplarError::RenderFailure(format!("This method expects {} arguments", #arg_count)).into();
            }
        }
    } else {
        quote!{}
    };

    let mut varset_tokens: Vec<TokenStream2> = vec![];

    varset_tokens.push(quote!{
        let #filter_in = match filter_in {
            #filter_in_type (val) => val.into(),
            _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into()
        };
    });

    match args.len() { 
        l if l > 1 => {
            for (arg, data_type) in args.iter().zip(data_types.iter()) {
                let dty = data_type.type_token();
                // println!("{} {:?}", i, data_type);
                varset_tokens.push(quote! {
                    let #arg = match filter_args.remove(0) {
                        #dty (val) => val.into(),
                        _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into(),
                    };
                });
            }
        }
        l if l == 1 => {
            let arg = args.first().unwrap();
            let dty = data_types.first().unwrap().type_token();
            varset_tokens.push(quote! {
                let #arg = match filter_args.into_result() {
                    Ok( #dty (val) ) => val.into(),
                    _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into(),
                };
            });
        }
        _ => {}
    }

    let mut fn_call_args: Vec<Ident> = vec![];
    for data_type in data_types.iter() {
        fn_call_args.push(data_type.name_token());
    }

    let return_tokens = match return_ty_string.as_str() {
        "Result" => quote! {
            match result {
                Ok(result) => result.into(),
                Err(e) => TemplarError::RenderFailure(format!("{:?}", e)).into(),
            }
        },
        "Option" => quote! {
            match result {
                Some(result) => result.into(),
                None => Data::empty(),
            }
        },
        _ => quote! {
            result.into()
        },
    };

    let gen = quote! {
        #orig_fn

        #vis fn #ident (filter_in: Data, filter_args: Data) -> Data {
            let filter_in: Document = match filter_in.into_result() {
                Ok(val) => val,
                Err(e) => return e.into(),
            };

            #arg_condition

            #( #varset_tokens )*

            let result: #return_ty = #inner_fn_ident ( #filter_in_name , #( #fn_call_args , )* ) ;
            #return_tokens
        }
    };

    // println!("{}", gen);

    gen.into()
}


pub fn impl_function(item_fn: &syn::ItemFn) -> TokenStream {
    let mut orig_fn = item_fn.clone();

    //function name
    let ident = &item_fn.sig.ident;
    let inner_fn = format!("__{}_inner__", ident);
    orig_fn.sig.ident = Ident::new(&inner_fn, ident.span());
    let inner_fn_ident = &orig_fn.sig.ident;
    // pub or not pub
    let vis = &item_fn.vis;
    // The arguments to the function
    let args: Vec<FnArg> = item_fn.sig.inputs.iter().cloned().collect();

    // Return type
    let return_ty = match &item_fn.sig.output {
        ReturnType::Type(_, ty) => ty,
        _ => return fail!(item_fn.sig.output.span(), "Return type is required"),
    };

    // string form of return type, only represents the last segment of the path. e.g. std::string::String would be just "String"
    let return_ty_string = match &**return_ty {
        Type::Path(TypePath { path, .. }) => path.segments.iter().last().unwrap().ident.to_string(),
        _ => return fail!(return_ty.span(), "This return type is not supported!"),
    };

    let data_types = match get_types_from_args(&args) {
        Ok(val) => val,
        Err(e) => return e.into(),
    };

    let arg_count: LitInt = LitInt::new(&args.len().to_string(), item_fn.sig.ident.span());

    let arg_condition = if args.len() > 1 {
        quote!{
            let mut args: Vec<Document> = match args {
                Document::Seq(val) => val,
                _ => return TemplarError::RenderFailure("Unexpected execution, arguments must be a sequence".to_string()).into(),
            };

            if args.len() != #arg_count {
                return TemplarError::RenderFailure(format!("This method expects {} arguments", #arg_count)).into();
            }
        }
    } else {
        quote!{}
    };

    let mut varset_tokens: Vec<TokenStream2> = vec![];

    match args.len() { 
        l if l > 1 => {
            for (arg, data_type) in args.iter().zip(data_types.iter()) {
                let dty = data_type.type_token();
                // println!("{} {:?}", i, data_type);
                varset_tokens.push(quote! {
                    let #arg = match args.remove(0) {
                        #dty (val) => val.into(),
                        _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into(),
                    };
                });
            }
        }
        l if l == 1 => {
            let arg = args.first().unwrap();
            let dty = data_types.first().unwrap().type_token();
            varset_tokens.push(quote! {
                let #arg = match args {
                    #dty (val) => val.into(),
                    _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into(),
                };
            });
        }
        _ => {}
    }

    let mut fn_call_args: Vec<Ident> = vec![];
    for data_type in data_types.iter() {
        fn_call_args.push(data_type.name_token());
    }

    let return_tokens = match return_ty_string.as_str() {
        "Result" => quote! {
            match result {
                Ok(result) => result.into(),
                Err(e) => TemplarError::RenderFailure(format!("{:?}", e)).into(),
            }
        },
        "Option" => quote! {
            match result {
                Some(result) => result.into(),
                None => Data::empty(),
            }
        },
        _ => quote! {
            result.into()
        },
    };

    let gen = quote! {
        #orig_fn

        #vis fn #ident (args: Data) -> Data {
            let args: Document = match args.into_result() {
                Ok(val) => val,
                Err(e) => return e.into(),
            };

            #arg_condition

            #( #varset_tokens )*

            let result: #return_ty = #inner_fn_ident ( #( #fn_call_args , )* ) ;
            #return_tokens
        }
    };

    // println!("{}", gen);

    gen.into()
}
