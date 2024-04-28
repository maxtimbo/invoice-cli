
#[macro_export]
macro_rules! select_entity {
    ($prompt:expr, $db:expr, $table:expr) => {
        {
            let list_result = $db.get_table($table)?;
            let selection: Result<i64, inquire::InquireError> = inquire::Select::new(
                $prompt,
                list_result
                .iter()
                .map(|sl| sl.id)
                .collect::<Vec<_>>())
                .prompt();
            selection
            //let entity = Default::default();
            //let query = $table(&entity);
            //let list_result = query.list_table(&$db)?;
            //let selection: Result<i64, inquire::InquireError> = inquire::Select::new($prompt, list_result).prompt();
            //selection
        }
    }
}

#[macro_export]
macro_rules! select_multiple_entities {
    ($prompt:expr, $db:expr, $table:expr) => {
        {
            let list_result = $db.get_table($table)?;
            let formatter: inquire::formatter::MultiOptionFormatter<i64> = &|options: &[inquire::list_option::ListOption<&i64>]| {
                format!("{} items selected", options.len())
            };
            let selection_result: Result<Vec<i64>, inquire::InquireError> = inquire::MultiSelect::new(
                $prompt,
                list_result
                .iter()
                .map(|sl| sl.id)
                .collect::<Vec<_>>())
                .with_formatter(formatter)
                .prompt();

            //let entity = Default::default();
            //let query = $table(&entity);
            //let list_result = query.list_table(&$db)?;
            //let formatter: inquire::formatter::MultiOptionFormatter<i64> = &|methods: &[inquire::list_option::ListOption<&i64>]| {
            //    format!("{} payment methods", methods.len())
            //};
            //let selection_result: Result<Vec<i64>, inquire::InquireError> = inquire::MultiSelect::new($prompt, list_result)
            //    .with_formatter(formatter)
            //    .prompt();
            selection_result
        }
    };
}

#[macro_export]
macro_rules! print_entries {
    ($entries:expr) => {
        for entry in $entries {
            println!("ID: {}, Name: {}", entry.id, entry.name);
        }
    };
}
