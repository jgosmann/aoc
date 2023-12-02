use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::{
    convert::TryFrom,
    fs::{self, FileType},
    path::Path,
};
use syn::{parse::Parse, parse_macro_input, token::Comma, Expr, Ident};

extern crate proc_macro;

struct Solver<'ident> {
    year: i32,
    day: u32,
    input_expr: &'ident Expr,
}

impl<'ident> ToTokens for Solver<'ident> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Solver {
            year,
            day,
            input_expr,
        } = *self;
        let year_mod = format_ident!("year{}", u32::try_from(year).expect("a non-negative year"));
        let day_mod = format_ident!("day{}", day);
        tokens.extend(quote!(
            (#year, #day) => Ok(Box::new(crate::solvers::#year_mod::#day_mod::SolverImpl::new(&#input_expr)?) as Box<dyn crate::solvers::Solver>),
        ));
    }
}

struct SolverDispatchInput {
    input_expr: Expr,
    year_ident: Ident,
    day_ident: Ident,
}

impl Parse for SolverDispatchInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let input_expr = Expr::parse(input)?;
        Comma::parse(input)?;
        let year_ident = Ident::parse(input)?;
        Comma::parse(input)?;
        let day_ident = Ident::parse(input)?;

        if !input.is_empty() {
            Comma::parse(input)?;
            if !input.is_empty() {
                return Err(syn::parse::Error::new(
                    input.span(),
                    "expected exactly 3 arguments",
                ));
            }
        }

        Ok(Self {
            input_expr,
            year_ident,
            day_ident,
        })
    }
}

struct File {
    file_type: FileType,
    file_name: String,
}

fn filter_by_file_type_and_name<P: AsRef<Path>, F: Fn(&File) -> bool + 'static>(
    path: P,
    predicate: F,
) -> impl Iterator<Item = File> {
    fs::read_dir(path.as_ref())
        .expect("cannot list directory")
        .filter_map(move |path| {
            let path = path.as_ref().expect("cannot read directory entry");
            let file_type = path.file_type().expect("cannot determine file type");
            let file_name = path
                .file_name()
                .into_string()
                .expect("invalid characters in file name");
            let file = File {
                file_type,
                file_name,
            };
            if predicate(&file) {
                Some(file)
            } else {
                None
            }
        })
}

#[proc_macro]
pub fn solver_dispatch(args: TokenStream) -> TokenStream {
    let SolverDispatchInput {
        input_expr,
        year_ident,
        day_ident,
    } = parse_macro_input!(args as SolverDispatchInput);

    let base_path = Path::new("src/solvers");
    let years = filter_by_file_type_and_name(&base_path, |file| {
        file.file_type.is_dir() && file.file_name.starts_with("year")
    })
    .map(|file| {
        file.file_name[4..]
            .parse::<i32>()
            .expect("directory names in format 'year<YYYY>'")
    });
    let years_with_days = years.flat_map(|year| {
        filter_by_file_type_and_name(base_path.join(format!("year{}", year)), |file| {
            file.file_type.is_file()
                && file.file_name.starts_with("day")
                && file.file_name.ends_with(".rs")
        })
        .map(move |file| {
            (
                year,
                file.file_name[3..file.file_name.len() - 3]
                    .parse::<u32>()
                    .expect("module names should be in format 'day<DD>.rs'"),
            )
        })
    });

    let solvers: Vec<Solver<'_>> = years_with_days
        .map(|(year, day)| Solver {
            year,
            day,
            input_expr: &input_expr,
        })
        .collect();

    quote!(
        match (#year_ident, #day_ident) {
            #(#solvers)*
            _ => Err(anyhow::anyhow!("no solver for day {} of year {}", day, year))
        }
    )
    .into()
}
