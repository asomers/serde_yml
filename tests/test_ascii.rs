#[cfg(test)]
mod tests {
    // Import necessary modules
    use figlet_rs::FIGfont;
    use serde_yml::macro_ascii;
    use serde_yml::{
        generators::ascii::{generate_ascii_art, load_standard_font},
        models::error_ascii_art::AsciiArtError,
    };

    // Test cases for generate_ascii_art function
    #[test]
    fn test_generate_ascii_art_success() {
        // Test successful generation of ASCII art from text
        let text = "Hello, world!";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_generate_ascii_art_empty_text() {
        // Test handling of empty text input
        let text = "";
        let result = generate_ascii_art(text);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AsciiArtError::ConversionError
        ));
    }

    #[test]
    fn test_generate_ascii_art_conversion_error() {
        // Test handling of conversion error for ASCII art generation
        let text = "\u{1F600}"; // Emoji character
        let result = generate_ascii_art(text);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AsciiArtError::ConversionError
        ));
    }

    #[test]
    fn test_generate_ascii_art_multiple_lines() {
        // Test generation of ASCII art from text with multiple lines
        let text = "Hello,\nworld!";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(ascii_art.contains('\n'));
    }

    #[test]
    fn test_generate_ascii_art_special_characters() {
        // Test generation of ASCII art from text with special characters
        let text = "!@#$%^&*()_+";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_generate_ascii_art_whitespace_only_text() {
        // Test handling of input containing only whitespace characters
        let text = " \t\n\r";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_generate_ascii_art_long_input() {
        // Test handling of very long input text
        let text = "a".repeat(1000);
        let result = generate_ascii_art(&text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_generate_ascii_art_alphanumeric_text() {
        // Test handling of input containing a mix of alphanumeric characters and whitespace
        let text = "Hello 123 !@#";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    // Test cases for load_standard_font function
    #[test]
    fn test_load_standard_font_success() {
        // Test loading of standard font for ASCII art generation
        let result = load_standard_font();
        assert!(result.is_ok());
    }

    // Test cases for FIGfont::from_file function
    #[test]
    fn test_load_standard_font_failure() {
        // Test handling of failure to load standard font
        // This test assumes there's a non-existent font file
        let result = FIGfont::from_file("non_existent_font_name.flf");
        assert!(result.is_err());
    }

    // Test cases for macro_ascii! macro
    #[test]
    fn test_macro_ascii_success() {
        // Test successful generation of ASCII art using macro
        let ascii_art = macro_ascii!("Hello, world!");
        assert!(!ascii_art.is_empty());
    }

    #[test]
    #[should_panic(
        expected = "Failed to generate ASCII art: Failed to convert text to ASCII art"
    )]
    fn test_macro_ascii_empty_text() {
        // Test handling of empty text input for macro_ascii! macro
        let _ = macro_ascii!("");
    }

    #[test]
    #[should_panic(
        expected = "Failed to generate ASCII art: Failed to convert text to ASCII art"
    )]
    fn test_macro_ascii_conversion_error() {
        // Test handling of conversion error for macro_ascii! macro
        let _ = macro_ascii!("\u{1F600}"); // Emoji character
    }

    #[test]
    fn test_macro_ascii_multiple_lines() {
        // Test generation of ASCII art from text with multiple lines using macro
        let ascii_art = macro_ascii!("Hello,\nworld!");
        assert!(ascii_art.contains('\n'));
    }

    #[test]
    fn test_macro_ascii_special_characters() {
        // Test generation of ASCII art from text with special characters using macro
        let ascii_art = macro_ascii!("!@#$%^&*()_+");
        assert!(!ascii_art.is_empty());
    }

    #[test]
    #[should_panic(expected = "Failed to generate ASCII art")]
    fn test_macro_ascii_error() {
        // Test handling of error when input contains non-ASCII text for macro_ascii! macro
        let input = "日本語"; // Non-ASCII text, will cause an error
        let _ = macro_ascii!(input);
    }

    // Test case for generate_ascii_art function with non-ASCII input
    #[test]
    fn test_generate_ascii_art_non_ascii_input() {
        // Test handling of non-ASCII input for generate_ascii_art function
        let text = "日本語"; // Non-ASCII text
        let result = generate_ascii_art(text);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AsciiArtError::ConversionError
        ));
    }
}
