use stylist::{StyleSource, css};

pub fn global_style() -> StyleSource {
    css!(
        r#"
            /* Root styles and variables */
            :root {
                /* Define your color scheme, fonts, and other root variables here */
                --primary-color: #hexcode; /* Primary color */
                --secondary-color: #hexcode; /* Secondary color */
                --background-color: #hexcode; /* Background color */
                --text-color: #hexcode; /* Text color */
                --font-family: 'YourFont', sans-serif; /* Font family */
            }

            /* Global styles */
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                font-family: var(--font-family);
                color: var(--text-color);
            }

            body {
                background-color: var(--background-color);
                line-height: 1.6;
            }

            /* Typography styles */
            h1, h2, h3, h4, h5, h6 {
                /* Add common styles for headers */
            }

            h1 {
                /* Specific styles for h1 tags */
            }

            /* Link styles */
            a {
                /* Style for links */
            }

            a:hover {
                /* Style for hover state of links */
            }

            /* Button styles */
            button {
                /* Style for buttons */
            }

            button:hover {
                /* Style for hover state of buttons */
            }

            /* Utility classes */
            .text-center {
                text-align: center;
            }

            .mt-1 {
                margin-top: 1rem;
            }

            .mb-1 {
                margin-bottom: 1rem;
            }

            /* Media Queries for responsive design */
            @media only screen and (max-width: 600px) {
                /* Responsive styles for small screens */
            }

            /* Additional custom component and element styles */
        "#
    )
}
