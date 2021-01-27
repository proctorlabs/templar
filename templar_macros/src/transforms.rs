// use crate::attr::*;
use crate::syn::spanned::Spanned;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;

use syn::{FnArg, Ident, LitInt, Pat, PatIdent, ReturnType, Type, TypePath};

macro_rules! data_types {
    ( $( $type:ident { $( $tail:tt )* } ( $( $tystr:tt )* ) ( $( $dorender:tt )* ) , )* ) => {
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
                            InnerData :: $( $tail )*
                        },
                    )*
                }.into()
            }

            pub fn do_render(&self) -> bool {
                match self {
                    $(
                        DataType:: $type (_) => $( $dorender )*,
                    )*
                }.into()
            }
        }
    };
}

data_types! {
    String  {String} ("String" | "&str") (true),
    Char    {Char}   ("char") (false),
    F64     {Number} ("f64") (false),
    F32     {Number} ("f32") (false),
    I128    {Number} ("i128" | "isize") (false),
    I64     {Number} ("i64") (false),
    I32     {Number} ("i32") (false),
    I16     {Number} ("i16") (false),
    I8      {Number} ("i8") (false),
    U128    {Number} ("u128" | "usize") (false),
    U64     {Number} ("u64") (false),
    U32     {Number} ("u32") (false),
    U16     {Number} ("u16") (false),
    U8      {Number} ("u8") (false),
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

fn safe_unwrap(var: FnArg, var_name: String, dt: DataType) -> TokenStream2 {
    let var_name = Ident::new(&var_name, var.span());
    if dt.do_render() {
        quote! {
            let #var = match Data::new( #var_name ).render() {
                Ok(v) => v,
                Err(e) => return e.into(),
            };
        }
    } else {
        quote! {
            let #var = match #var_name .cast() {
                Some(v) => v,
                None => return TemplarError::RenderFailure(format!("Unexpected type in argument2")).into()
            };
        }
    }
}

fn arg_condition(args: &[FnArg], args_name: &str) -> TokenStream2 {
    if args.len() > 1 {
        let args_name = Ident::new(args_name, args[0].span());
        let arg_count: LitInt = LitInt::new(&args.len().to_string(), args[0].span());
        quote! {
            let mut #args_name : Vec<InnerData> = match #args_name .into_inner() {
                InnerData::Seq(val) => val,
                InnerData::Err(e) => return e.into(),
                _ => return TemplarError::RenderFailure("Unexpected execution, arguments must be a sequence".to_string()).into(),
            };

            if #args_name .len() != #arg_count {
                return TemplarError::RenderFailure(format!("This method expects {} arguments", #arg_count)).into();
            }
        }
    } else {
        quote! {}
    }
}

fn return_tokens(item_fn: &syn::ItemFn) -> (&std::boxed::Box<syn::Type>, TokenStream2) {
    // Return type
    let return_ty = match &item_fn.sig.output {
        ReturnType::Type(_, ty) => ty,
        _ => panic!("Return type is required"),
    };

    // string form of return type, only represents the last segment of the path. e.g. std::string::String would be just "String"
    let return_ty_string = match &**return_ty {
        Type::Path(TypePath { path, .. }) => path.segments.iter().last().unwrap().ident.to_string(),
        _ => {
            return (
                return_ty,
                fail!(return_ty.span(), "This return type is not supported!"),
            )
        }
    };

    (
        return_ty,
        match return_ty_string.as_str() {
            "Result" => quote! {
                match result {
                    Ok(result) => Data::new(result),
                    Err(e) => TemplarError::RenderFailure(format!("{:?}", e)).into(),
                }
            },
            "Option" => quote! {
                match result {
                    Some(result) => Data::new(result),
                    None => Data::empty(),
                }
            },
            _ => quote! {
                Data::new(result)
            },
        },
    )
}

fn push_set_variables(
    target: &mut Vec<TokenStream2>,
    args_name: &str,
    data_types: &[DataType],
    args: &[FnArg],
) {
    match args.len() {
        l if l > 1 => {
            let args_name = Ident::new(args_name, args[0].span());
            for (arg, data_type) in args.iter().zip(data_types.iter()) {
                let dty = data_type.type_token();
                let do_render = data_type.do_render();
                if do_render {
                    target.push(quote! {
                        let #arg = match Data::new(#args_name .remove(0)).render() {
                            Ok(val) => val,
                            Err(e) => return e.into(),
                        };
                    });
                } else {
                    target.push(quote! {
                        let #arg = match #args_name .remove(0) {
                            #dty (val) => val.into(),
                            _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into(),
                        };
                    });
                }
            }
        }
        l if l == 1 => {
            let args_name = Ident::new(&args_name, args[0].span());
            let arg = args.first().unwrap();
            let dty = data_types.first().unwrap();
            let do_render = dty.do_render();
            let dty = dty.type_token();
            if do_render {
                target.push(quote! {
                    let #arg = match #args_name .render() {
                        Ok(val) => val,
                        Err(e) => return e.into(),
                    };
                });
            } else {
                target.push(quote! {
                    let #arg = match #args_name .into_inner() {
                        #dty (val) => val.into(),
                        _ => return TemplarError::RenderFailure(format!("Unexpected type in argument")).into(),
                    };
                });
            }
        }
        _ => {}
    }
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
    let (return_ty, return_tokens) = return_tokens(item_fn);

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
    let filter_in_name = filter_in_dt.name_token();

    let arg_condition = arg_condition(&args, "filter_args");

    let mut varset_tokens: Vec<TokenStream2> =
        vec![safe_unwrap(filter_in, "filter_in".into(), filter_in_dt)];
    push_set_variables(&mut varset_tokens, "filter_args", &data_types, &args);

    let mut fn_call_args: Vec<Ident> = vec![];
    for data_type in data_types.iter() {
        fn_call_args.push(data_type.name_token());
    }

    let gen = quote! {
        #orig_fn

        #vis fn #ident (filter_in: Data, filter_args: Data) -> Data {
            let filter_in: InnerData = match filter_in .into_inner() {
                InnerData::Err(e) => return e.into(),
                val => val,
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
    let (return_ty, return_tokens) = return_tokens(item_fn);

    let data_types = match get_types_from_args(&args) {
        Ok(val) => val,
        Err(e) => return e.into(),
    };

    let arg_condition = arg_condition(&args, "args");
    let mut varset_tokens: Vec<TokenStream2> = vec![];
    push_set_variables(&mut varset_tokens, "args", &data_types, &args);

    let mut fn_call_args: Vec<Ident> = vec![];
    for data_type in data_types.iter() {
        fn_call_args.push(data_type.name_token());
    }

    let gen = quote! {
        #orig_fn

        #vis fn #ident (args: Data) -> Data {
            #arg_condition

            #( #varset_tokens )*

            let result: #return_ty = #inner_fn_ident ( #( #fn_call_args , )* ) ;
            #return_tokens
        }
    };

    // println!("{}", gen);

    gen.into()
}
