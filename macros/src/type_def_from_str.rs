use stellar_xdr::{
    SpecTypeDef, SpecTypeMap, SpecTypeOption, SpecTypeSet, SpecTypeTuple, SpecTypeUdt, SpecTypeVec,
};
use syn::{GenericArgument, Path, PathArguments, PathSegment, Type, TypePath, TypeTuple};

// TODO: Remove user-defined types from SpecTypeDef and treat separately.

pub fn type_def_from_str(t: &Type) -> SpecTypeDef {
    match t {
        Type::Path(TypePath {
            qself: None,
            path: Path { segments, .. },
        }) => {
            match segments.last() {
                Some(PathSegment {
                    ident,
                    arguments: PathArguments::None,
                }) => {
                    #[allow(clippy::match_same_arms)]
                    match &ident.to_string()[..] {
                        "u64" => SpecTypeDef::U64,
                        "i64" => SpecTypeDef::I64,
                        "u32" => SpecTypeDef::U32,
                        "i32" => SpecTypeDef::I32,
                        "bool" => SpecTypeDef::Bool,
                        "Symbol" => SpecTypeDef::Symbol,
                        "Bitset" => SpecTypeDef::Bitset,
                        "Status" => SpecTypeDef::Status,
                        "Binary" => SpecTypeDef::Binary,
                        s => SpecTypeDef::Udt(SpecTypeUdt {
                            name: s.try_into().unwrap(), // TODO: Write compiler error.
                        }),
                    }
                }
                Some(PathSegment {
                    ident,
                    arguments: PathArguments::AngleBracketed(args),
                }) => {
                    let args = args.args.iter().collect::<Vec<&GenericArgument>>();
                    #[allow(clippy::match_same_arms)]
                    match &ident.to_string()[..] {
                        "Option" => {
                            let t = match args.as_slice() {
                                [GenericArgument::Type(t)] => t,
                                [..] => unimplemented!(), // TODO: Write compiler error.
                            };
                            SpecTypeDef::Option(Box::new(SpecTypeOption {
                                value_type: Box::new(type_def_from_str(t)),
                            }))
                        }
                        "Vec" => {
                            let t = match args.as_slice() {
                                [GenericArgument::Type(t)] => t,
                                [..] => unimplemented!(), // TODO: Write compiler error.
                            };
                            SpecTypeDef::Vec(Box::new(SpecTypeVec {
                                element_type: Box::new(type_def_from_str(t)),
                            }))
                        }
                        "Set" => {
                            let t = match args.as_slice() {
                                [GenericArgument::Type(t)] => t,
                                [..] => unimplemented!(), // TODO: Write compiler error.
                            };
                            SpecTypeDef::Set(Box::new(SpecTypeSet {
                                element_type: Box::new(type_def_from_str(t)),
                            }))
                        }
                        "Map<K, V>" => {
                            let (k, v) = match args.as_slice() {
                                [GenericArgument::Type(k), GenericArgument::Type(v)] => (k, v),
                                [..] => unimplemented!(), // TODO: Write compiler error.
                            };
                            SpecTypeDef::Map(Box::new(SpecTypeMap {
                                key_type: Box::new(type_def_from_str(k)),
                                value_type: Box::new(type_def_from_str(v)),
                            }))
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            }
        }
        Type::Tuple(TypeTuple { elems, .. }) => SpecTypeDef::Tuple(Box::new(SpecTypeTuple {
            value_types: elems
                .iter()
                .map(type_def_from_str)
                .collect::<Vec<SpecTypeDef>>() // TODO: Implement conversion to VecM from iters to omit this collect.
                .try_into()
                .unwrap(),
        })),
        _ => unimplemented!(),
    }
}