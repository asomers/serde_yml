/// Macro to simplify the match logic for file generation.
#[macro_export]
macro_rules! generate_file {
    ($file_type:expr, $value:expr, $generator:expr) => {
        if !$value.trim().is_empty() {
            if let Err(err) = $generator($value) {
                eprintln!(
                    "Error generating {} file: {}",
                    $file_type, err
                );
            }
        }
    };
}
