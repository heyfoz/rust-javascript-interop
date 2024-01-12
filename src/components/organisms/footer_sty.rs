// Importing style utilities from the stylist crate
use stylist::{ style, Style };

// Define a function to apply styles and structure CSS elements in Rust code for a component.
// Note: Use block comments (/* ... */) within the r#""# section for CSS comments to avoid compile errors.
pub fn style() -> Style {
    style!(
        r#"
            /* Main layout container */
            .main-container {
                /* Add styles for overall layout like display, alignment */
            }

            /* Footer container styles */
            .footer {
                /* Style the footer, e.g., positioning, padding */
            }

            /* Image styling within a link */
            .footer a > img {
                /* Define image styles like size, border, etc. */
            }

            /* Hover effect for images */
            .footer a:hover > img {
                /* Add hover styles, like border color changes */
            }

            /* Line or separator styles */
            .footer a > .line {
                /* Style for lines or separators, e.g., width, color */
            }
        "#
    ).unwrap() // Unwrap the result to handle potential errors
}
