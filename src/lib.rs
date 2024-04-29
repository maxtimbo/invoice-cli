
#[macro_export]
macro_rules! select_entity {
    ($prompt:expr, $db:expr, $table:expr) => {
        {
            use inquire::{Select, InquireError};

            let list_result = $db.get_table($table)?;
            let selection: Result<i64, InquireError> = Select::new(
                $prompt,
                list_result
                .iter()
                .map(|sl| sl.id)
                .collect::<Vec<_>>())
                .prompt();
            selection
        }
    }
}

#[macro_export]
macro_rules! select_multiple_entities {
    ($prompt:expr, $db:expr, $table:expr) => {
        {
            use inquire::{
                formatter::MultiOptionFormatter,
                list_option::ListOption,
                validator::{Validation, ErrorMessage},
                InquireError,
                MultiSelect
            };
            let list_result = $db.get_table($table)?;
            let formatter: MultiOptionFormatter<i64> = &|options: &[ListOption<&i64>]| {
                format!("{} items selected", options.len())
            };
            let validator = |a: &[ListOption<&i64>]| {
                if a.len() > 1 {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(ErrorMessage::Custom("Must make a selection".to_string())))
                }
            };
            let selection_result: Result<Vec<i64>, InquireError> = MultiSelect::new(
                $prompt,
                list_result
                .iter()
                .map(|sl| sl.id)
                .collect::<Vec<_>>())
                .with_formatter(formatter)
                .with_validator(validator)
                .prompt();
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
