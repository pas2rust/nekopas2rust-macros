use super::prelude::*;

pub fn sql_app(input: &DeriveInput) -> TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;

    let struct_ident = get_struct_name(input);
    let table_name = struct_ident.to_string().to_lowercase();
    let impl_block = get_impl(input);

    let columns_vec: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    let placeholders_vec: Vec<String> = (1..=columns_vec.len()).map(|i| format!("${i}")).collect();

    let insert_params_tokens: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            quote! { (&self.#ident) as &(dyn tokio_postgres::types::ToSql + Sync) }
        })
        .collect();

    let columns = columns_vec.join(", ");
    let placeholders = placeholders_vec.join(", ");
    let sql_select_all = format!("SELECT {columns} FROM {table_name}");
    let sql_insert = format!("INSERT INTO {table_name} ({columns}) VALUES ({placeholders})");

    let mut select_by_fns = Vec::new();
    let mut delete_by_fns = Vec::new();
    let mut update_by_fns = Vec::new();

    for f in fields.iter() {
        let ident = f.ident.as_ref().unwrap();
        let field_name = ident.to_string();

        let sel_fn = format_ident!("sql_select_by_{}", field_name);
        let del_fn = format_ident!("sql_delete_by_{}", field_name);
        let upd_fn = format_ident!("sql_update_by_{}", field_name);

        let sql_select_by = format!("SELECT {columns} FROM {table_name} WHERE {field_name} = $1");

        select_by_fns.push(quote! {
            fn #sel_fn<'a>(&'a self)
                -> (&'static str, Vec<&'a (dyn tokio_postgres::types::ToSql + Sync)>)
            {
                (
                    #sql_select_by,
                    vec![ (&self.#ident) as &(dyn tokio_postgres::types::ToSql + Sync) ]
                )
            }
        });

        let sql_delete_by = format!("DELETE FROM {table_name} WHERE {field_name} = $1");

        delete_by_fns.push(quote! {
            fn #del_fn<'a>(&'a self)
                -> (&'static str, Vec<&'a (dyn tokio_postgres::types::ToSql + Sync)>)
            {
                (
                    #sql_delete_by,
                    vec![ (&self.#ident) as &(dyn tokio_postgres::types::ToSql + Sync) ]
                )
            }
        });

        let set_columns: Vec<String> = columns_vec
            .iter()
            .filter(|c| *c != &field_name)
            .cloned()
            .collect();

        let (set_clause, set_param_tokens) = if set_columns.is_empty() {
            (
                format!("{field_name} = {field_name}"),
                Vec::<proc_macro2::TokenStream>::new(),
            )
        } else {
            let set_parts: Vec<String> = set_columns
                .iter()
                .enumerate()
                .map(|(i, col)| format!("{col} = ${}", i + 1))
                .collect();

            let mut toks = Vec::new();
            for col in &set_columns {
                let tok = fields
                    .iter()
                    .find(|ff| ff.ident.as_ref().unwrap() == col.as_str())
                    .map(|ff| {
                        let id = ff.ident.as_ref().unwrap();
                        quote! { (&self.#id) as &(dyn tokio_postgres::types::ToSql + Sync) }
                    })
                    .unwrap();
                toks.push(tok);
            }

            (set_parts.join(", "), toks)
        };

        let where_idx = if set_param_tokens.is_empty() {
            1
        } else {
            set_param_tokens.len() + 1
        };

        let sql_update_by =
            format!("UPDATE {table_name} SET {set_clause} WHERE {field_name} = ${where_idx}");

        let mut vec_tokens = set_param_tokens.clone();
        vec_tokens.push(quote! { (&self.#ident) as &(dyn tokio_postgres::types::ToSql + Sync) });

        update_by_fns.push(quote! {
            fn #upd_fn<'a>(&'a self)
                -> (&'static str, Vec<&'a (dyn tokio_postgres::types::ToSql + Sync)>)
            {
                (
                    #sql_update_by,
                    {
                        let mut v = Vec::new();
                        #( v.push(#vec_tokens); )*
                        v
                    }
                )
            }
        });
    }

    quote! {
       impl #impl_block {
            fn sql_insert<'a>(&'a self)
                -> (&'static str, Vec<&'a (dyn tokio_postgres::types::ToSql + Sync)>)
            {
                (#sql_insert, vec![ #(#insert_params_tokens),* ])
            }

            fn sql_select_all<'a>(&'a self)
                -> (&'static str, Vec<&'a (dyn tokio_postgres::types::ToSql + Sync)>)
            {
                (#sql_select_all, Vec::new())
            }
            #(#select_by_fns)*
            #(#delete_by_fns)*
            #(#update_by_fns)*
       }
    }
}
