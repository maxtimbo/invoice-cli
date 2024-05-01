
#[macro_export]
macro_rules! select_entity {
    ($prompt:expr, $db:expr, $table:expr) => {
        {
            use inquire::{Select, InquireError};

            let list_result = $db.get_table($table)?;
            let options = list_result.iter().map(|sl| format!("{} - {}", sl.id, sl.name)).collect::<Vec<_>>();
            let selection: Result<i64, InquireError> = Select::new($prompt, options)
                .prompt()
                .map(|answer| {
                    answer.split(" - ").next().unwrap().parse::<i64>().unwrap()
                });
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
            let options = list_result.iter().map(|sl| format!("{} - {}", sl.id, sl.name)).collect::<Vec<_>>();

            let formatter: MultiOptionFormatter<String> = &|options: &[ListOption<&String>]| {
                format!("{} items selected", options.len())
            };
            let validator = |a: &[ListOption<&String>]| {
                if a.len() > 0 {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(ErrorMessage::Custom("Must make a selection".to_string())))
                }
            };
            let selection_result: Result<Vec<i64>, InquireError> = MultiSelect::new($prompt, options)
                .with_formatter(formatter)
                .with_validator(validator)
                .prompt()
                .map(|answers| {
                    answers.iter().map(|answer| {
                        answer.split(" - ").next().unwrap().parse::<i64>().unwrap()
                    }).collect::<Vec<_>>()
                });
            selection_result
        }
    };
}
