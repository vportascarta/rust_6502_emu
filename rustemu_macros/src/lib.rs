use proc_macro::TokenStream;
use quote::quote;
use syn::Variant;

#[proc_macro_derive(EmuInstruction, attributes(opcode, asmstr, addrmode))]
pub fn generate_vm_instruction_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_instruction_struct(&ast, false)
}

#[proc_macro_derive(EmuInstructionStrict, attributes(opcode, asmstr, addrmode))]
pub fn generate_vm_instruction_impl_strict(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_instruction_struct(&ast, true)
}

fn get_opcode(x: &Variant) -> u8 {
    for attr in x.attrs.iter() {
        if attr.path().is_ident("opcode") {
            let value: syn::LitInt = attr.parse_args().unwrap();
            return value.base10_parse().unwrap();
        }
    }
    0
}

fn get_asmstr(x: &Variant) -> String {
    for attr in x.attrs.iter() {
        if attr.path().is_ident("asmstr") {
            let value: syn::LitStr = attr.parse_args().unwrap();
            return value.value()
        }
    }
    "".to_string()
}

fn get_addrmode(x: &Variant) -> String {
    for attr in x.attrs.iter() {
        if attr.path().is_ident("addrmode") {
            let value: syn::LitStr = attr.parse_args().unwrap();
            return value.value()
        }
    }
    "".to_string()
}

fn get_type_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(x) = ty {
        x.path
            .segments
            .iter()
            .map(|x| x.ident.to_string())
            .collect()
    } else {
        panic!("don't know how to handle this type")
    }
}

fn get_operand_type(x: &Variant) -> Option<String> {
    match &x.fields {
        syn::Fields::Named(fields) => fields.named.iter().map(|x| get_type_name(&x.ty)).nth(0),
        syn::Fields::Unnamed(fields) => fields.unnamed.iter().map(|x| get_type_name(&x.ty)).nth(0),
        syn::Fields::Unit => None,
    }
}

fn impl_instruction_struct(ast: &syn::ItemEnum, strict_mode: bool) -> TokenStream {
    let allowed_addr_modes = vec!["imp","imm","abs","abi","abx","aby","zpm","zpx", "zpy","zxi","zyi","rel"];
    let mut already_parse_opcode: Vec<u8> = Vec::new();

    let mut field_size: Vec<_> = Vec::new();
    let mut field_to_binary: Vec<_> = Vec::new();
    let mut field_from_binary: Vec<_> = Vec::new();
    let mut field_to_string: Vec<_> = Vec::new();
    let mut field_from_string: Vec<_> = Vec::new();

    for x in ast.variants.iter() {
        let field_name = &x.ident;
        let field_opcode: u8 = get_opcode(x);
        let _field_asmstr: String = get_asmstr(x);
        let field_addrmode: String = get_addrmode(x);
        let field_param_type = get_operand_type(x);

        if strict_mode {
            if !allowed_addr_modes.contains(&field_addrmode.as_str()) {
                panic!("The address mode of {} is not correct", field_name)
            }

            if already_parse_opcode.contains(&field_opcode) {
                panic!("The opcode of {} has already been parsed", field_name)
            }
        }
        already_parse_opcode.push(field_opcode);

        match field_param_type {
            None => {
                field_size.push(quote! {
                    Instruction::#field_name => 1
                });
                field_from_binary.push(quote! {
                    #field_opcode => Ok(Self::#field_name)
                });
                field_to_binary.push(quote! {
                    Instruction::#field_name => vec![#field_opcode]
                });
                field_to_string.push(quote! {
                    Instruction::#field_name => write!(f, stringify!(#field_name))
                });
                field_from_string.push(quote! {
                    stringify!(#field_name) => {
                        if tokens.len() == 1 {
                            Ok(Self::#field_name)
                        } else {
                            Err(ParsingError::BlockingError(format!(
                                "Not right number of parameters for {}", stringify!(#field_name)
                            )))
                        }
                    }
                });
            }
            Some(ty) if ty == "u8" => {
                field_size.push(quote! {
                    Instruction::#field_name(_) => 2
                });
                field_from_binary.push(quote! {
                    #field_opcode => {
                        if value.len() < 2 {
                            Err(format!("Not right number of operands for {}", stringify!(#field_name)))
                        } else {
                            Ok(Self::#field_name(value[1]))
                        }
                    }
                });
                field_to_binary.push(quote! {
                    Instruction::#field_name(op) => vec![#field_opcode, op]
                });
                field_from_string.push(quote! {
                    stringify!(#field_name) => {
                        if tokens.len() == 2 {
                            Ok(Self::#field_name(
                                try_parse_numeric_u8(tokens[1])
                                    .map_err(|x| ParsingError::BlockingError(x))?,
                            ))
                        } else {
                            Err(ParsingError::BlockingError(format!(
                                "Not right number of parameters for {}", stringify!(#field_name)
                            )))
                        }
                    }
                });
                field_to_string.push(quote! {
                    Instruction::#field_name(op) => write!(f, "{} ${:02x}", stringify!(#field_name), op)
                });
            }
            Some(ty) if ty == "u16" => {
                field_size.push(quote! {
                    Instruction::#field_name(_) => 3
                });
                field_from_binary.push(quote! {
                    #field_opcode => {
                        if value.len() < 3 {
                            Err(format!("Not right number of operands for {}", stringify!(#field_name)))
                        } else {
                            let operand: u16 = ((value[2] as u16) << 8) | (value[1] as u16);
                            Ok(Self::#field_name(operand))
                        }
                    }
                });
                field_to_binary.push(quote! {
                    Instruction::#field_name(op) => {
                        let low_op: u8 = (op & 0xFF) as u8;
                        let high_op: u8 = ((op & 0xFF00) >> 8) as u8;
                        vec![#field_opcode, low_op, high_op]
                    }
                });
                field_from_string.push(quote! {
                    stringify!(#field_name) => {
                        if tokens.len() == 2 {
                            Ok(Self::#field_name(
                                try_parse_numeric_u16(tokens[1])
                                    .map_err(|x| ParsingError::BlockingError(x))?,
                            ))
                        } else {
                            Err(ParsingError::BlockingError(format!(
                                "Not right number of parameters for {}", stringify!(#field_name)
                            )))
                        }
                    }
                });
                field_to_string.push(quote! {
                    Instruction::#field_name(op) => write!(f, "{} ${:04x}", stringify!(#field_name), op)
                });
            }
            _ => todo!(),
        }
    }

    quote! {
        impl Instruction {
            pub fn size(self) -> usize {
                match self {
                    #(#field_size,)*
                }
            }
        }

        impl TryFrom<&[u8]> for Instruction {
            type Error = String;
        
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                if value.len() == 0 {
                    return Err(format!("You have passed an empty array"));
                }
        
                match value[0] {
                    #(#field_from_binary,)*
                    _ => Err(format!(
                        "You have passed an unknown instruction : 0x{:02x}",
                        value[0]
                    ))
                }
            }
        }

        impl Into<Vec<u8>> for Instruction {
            fn into(self) -> Vec<u8> {
                match self {
                    #(#field_to_binary,)*
                }
            }
        }

        impl FromStr for Instruction {
            type Err = ParsingError;
        
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let first_char = value
                    .chars()
                    .nth(0)
                    .ok_or(ParsingError::NonBlockingError(format!(
                        "You have passed an empty string"
                    )))?;
        
                if first_char == ';' {
                    return Err(ParsingError::NonBlockingError(format!("This is a comment")));
                }
        
                let tokens: Vec<&str> = value.split(' ').filter(|x| !x.is_empty()).collect();
        
                if tokens.len() == 0 {
                    return Err(ParsingError::NonBlockingError(format!(
                        "You have passed an empty string"
                    )));
                }
        
                match tokens[0] {
                    #(#field_from_string,)*
                    _ => Err(ParsingError::BlockingError(format!(
                        "You have passed an unknown instruction : {}",
                        tokens[0]
                    ))),
                }
            }
        }

        impl fmt::Display for Instruction {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    #(#field_to_string,)*
                }
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum ParsingError {
            NonBlockingError(String),
            BlockingError(String),
        }

        fn try_parse_numeric_u8(num_str: &str) -> Result<u8, String> {
            let first_char = num_str
                .chars()
                .nth(0)
                .ok_or(format!("You have passed an empty string"))?;
        
            match first_char {
                '$' => u8::from_str_radix(&num_str[1..], 16)
                    .map_err(|_| format!("Error converting u8 from hex string")),
                '%' => u8::from_str_radix(&num_str[1..], 2)
                    .map_err(|_| format!("Error converting u8 from binary string")),
                _ => {
                    u8::from_str_radix(num_str, 10).map_err(|_| format!("Error converting u8 from string"))
                }
            }
        }
        
        fn try_parse_numeric_u16(num_str: &str) -> Result<u16, String> {
            let first_char = num_str
                .chars()
                .nth(0)
                .ok_or(format!("You have passed an empty string"))?;
        
            match first_char {
                '$' => u16::from_str_radix(&num_str[1..], 16)
                    .map_err(|_| format!("Error converting u16 from hex string")),
                '%' => u16::from_str_radix(&num_str[1..], 2)
                    .map_err(|_| format!("Error converting u16 from binary string")),
                _ => u16::from_str_radix(num_str, 10)
                    .map_err(|_| format!("Error converting u16 from string")),
            }
        }
    }
    .into()
}
