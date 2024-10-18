//! # ByteColor Crate
//!
//! The `ByteColor` crate offers a versatile and comprehensive trait, `ByteColor`, designed to enhance terminal output
//! by applying ANSI color codes and text styles to various data types. This trait abstracts the complexities of ANSI
//! escape codes, providing a straightforward and intuitive interface for developers to create visually appealing and
//! informative console outputs.
//!
//! ## Table of Contents
//!
//! - [Overview](#overview)
//! - [Features](#features)
//! - [Getting Started](#getting-started)
//! - [Usage](#usage)
//!   - [Colorizing Primitive Types](#colorizing-primitive-types)
//!   - [Colorizing Strings and Byte Arrays](#colorizing-strings-and-byte-arrays)
//! - [Trait Definition](#trait-definition)
//! - [Implementation Details](#implementation-details)
//!   - [Macro Usage](#macro-usage)
//!   - [Handling Byte Slices and Vectors](#handling-byte-slices-and-vectors)
//! - [Best Practices](#best-practices)
//! - [Caveats and Considerations](#caveats-and-considerations)
//! - [Extensibility](#extensibility)
//! - [Further Reading](#further-reading)
//! - [License](#license)
//!
//! ## Overview
//!
//! In the realm of command-line applications, enhancing the readability and aesthetic appeal of terminal outputs is paramount.
//! The `ByteColor` trait serves as a robust solution for applying colors and text styles to various data types,
//! including primitive numeric types, strings, and byte arrays. By leveraging this trait, developers can effortlessly
//! enrich their application's console output, making it more engaging and user-friendly.
//!
//! ## Features
//!
//! - **Comprehensive Color Methods:** Apply standard ANSI colors such as red, green, yellow, magenta, cyan, and blue.
//! - **Text Styling:** Enhance text with styles like bold, underline, and blink.
//! - **Custom RGB Colors:** Utilize custom RGB tuples for precise color control.
//! - **256-Color Support:** Apply colors from the 256-color ANSI palette using color codes.
//! - **Broad Type Support:** Implementations available for primitive numeric types, string slices (`&str`), `String`, byte slices (`&[u8]`), and byte vectors (`Vec<u8>`).
//! - **Efficient Implementations:** Utilize Rust's macro system to minimize boilerplate and ensure consistency across implementations.
//!
//! ## Getting Started
//!
//! To integrate `ByteColor` into your project, follow these simple steps:
//!
//! 1. **Add Dependency:**
//!
//!    Add the `bytescolor` crate to your `Cargo.toml`:
//!
//!    ```toml
//!    [dependencies]
//!    bytescolor = "0.1.0" // Replace with the latest version
//!    ```
//!
//! 2. **Import the Trait:**
//!
//!    In your Rust file, import the `ByteColor` trait:
//!
//!    ```rust
//!    use bytescolor::ByteColor;
//!    ```
//!
//! ## Usage
//!
//! The `ByteColor` trait can be seamlessly applied to various data types to enhance their display in the terminal.
//!
//! ### Colorizing Primitive Types
//!
//! ```rust
//! use bytescolor::ByteColor;
//!
//! fn main() {
//!     let number: u32 = 100;
//!     let hexadecimal: u16 = 0xFF;
//!     let large_number: usize = 1_000_000;
//!
//!     println!("Number: {}", number.green());
//!     println!("Hex: {}", hexadecimal.yellow());
//!     println!("Count: {}", large_number.magenta());
//! }
//! ```
//!
//! **Output:**
//! - "Number: 100" displayed in green.
//! - "Hex: 255" displayed in yellow.
//! - "Count: 1000000" displayed in magenta.
//!
//! ### Colorizing Strings and Byte Arrays
//!
//! ```rust
//! use bytescolor::ByteColor;
//!
//! fn main() {
//!     let message: &str = "Hello, World!";
//!     let dynamic_message: String = String::from("Dynamic Hello");
//!     let byte_slice: &[u8] = b"Byte Slice";
//!     let byte_vector: Vec<u8> = vec![66, 121, 116, 101, 32, 86, 101, 99];
//!
//!     println!("{}", message.red());
//!     println!("{}", dynamic_message.blue());
//!     println!("{}", byte_slice.cyan());
//!     println!("{}", byte_vector.bold());
//! }
//! ```
//!
//! **Output:**
//! - "Hello, World!" displayed in red.
//! - "Dynamic Hello" displayed in blue.
//! - "Byte Slice" displayed in cyan.
//! - "Byte Vec" displayed in bold.
//!
//! ## Trait Definition
//!
//! The `ByteColor` trait defines a suite of methods for applying ANSI color codes and text styles to various types.
//! Each method returns a `String` with the appropriate ANSI escape sequences encapsulating the original value.
//!
//! ```plaintext
//! /// The `ByteColor` trait provides methods to apply ANSI colors and text styles to various types.
//! /// Each method returns a `String` with the corresponding ANSI escape codes applied.
//! pub trait ByteColor {
//!     /// Applies red color to the text.
//!     fn red(&self) -> String;
//!
//!     /// Applies green color to the text.
//!     fn green(&self) -> String;
//!
//!     /// Applies yellow color to the text.
//!     fn yellow(&self) -> String;
//!
//!     /// Applies magenta color to the text.
//!     fn magenta(&self) -> String;
//!
//!     /// Applies cyan color to the text.
//!     fn cyan(&self) -> String;
//!
//!     /// Applies blue color to the text.
//!     fn blue(&self) -> String;
//!
//!     /// Makes the text bold.
//!     fn bold(&self) -> String;
//!
//!     /// Underlines the text.
//!     fn underline(&self) -> String;
//!
//!     /// Makes the text blink.
//!     fn blink(&self) -> String;
//!
//!     /// Applies a custom RGB color to the text.
//!     ///
//!     /// # Parameters
//!     ///
//!     /// - `rgb`: A tuple representing the red, green, and blue components of the color.
//!     fn rgb(&self, rgb: (u8, u8, u8)) -> String;
//!
//!     /// Applies a custom 256-color palette color to the text using a color code.
//!     ///
//!     /// # Parameters
//!     ///
//!     /// - `code`: An ANSI color code ranging from 0 to 255.
//!     fn color(&self, code: u8) -> String;
//! }
//! ```
//!
//! ## Implementation Details
//!
//! The `ByteColor` trait is implemented for a variety of types to ensure flexibility and broad usage. These implementations
//! leverage Rust's powerful macro system to reduce redundancy and maintain consistency across different type implementations.
//!
//! ### Macro Usage
//!
//! To efficiently implement the `ByteColor` trait for multiple primitive numeric types, a macro is employed. This macro
//! iterates over a list of types and generates the necessary trait implementations, each applying the appropriate ANSI escape codes.
//!
//! **Macro Definition:**
//!
//! ```plaintext
//! macro_rules! impl_colorize_for_primitive {
//!     ($($t:ty),*) => {
//!         $(
//!             impl ByteColor for $t {
//!                 fn red(&self) -> String {
//!                     format!("\x1b[31m{}\x1b[0m", self)
//!                 }
//!
//!                 fn green(&self) -> String {
//!                     format!("\x1b[32m{}\x1b[0m", self)
//!                 }
//!
//!                 fn yellow(&self) -> String {
//!                     format!("\x1b[33m{}\x1b[0m", self)
//!                 }
//!
//!                 fn magenta(&self) -> String {
//!                     format!("\x1b[35m{}\x1b[0m", self)
//!                 }
//!
//!                 fn cyan(&self) -> String {
//!                     format!("\x1b[36m{}\x1b[0m", self)
//!                 }
//!
//!                 fn blue(&self) -> String {
//!                     format!("\x1b[34m{}\x1b[0m", self)
//!                 }
//!
//!                 fn bold(&self) -> String {
//!                     format!("\x1b[1m{}\x1b[0m", self)
//!                 }
//!
//!                 fn underline(&self) -> String {
//!                     format!("\x1b[4m{}\x1b[0m", self)
//!                 }
//!
//!                 fn blink(&self) -> String {
//!                     format!("\x1b[5m{}\x1b[0m", self)
//!                 }
//!
//!                 fn rgb(&self, color: (u8, u8, u8)) -> String {
//!                     format!("\x1b[38;2;{};{};{}m{}\x1b[0m", color.0, color.1, color.2, self)
//!                 }
//!
//!                 fn color(&self, color_code: u8) -> String {
//!                     format!("\x1b[38;5;{}m{}\x1b[0m", color_code, self)
//!                 }
//!             }
//!         )*
//!     };
//! }
//!
//! // Apply the macro to primitive types
//! impl_colorize_for_primitive!(u8, u16, u32, u64, i8, i16, i32, i64, usize);
//! ```
//!
//! **Explanation:**
//!
//! - The `impl_colorize_for_primitive!` macro takes a list of primitive types and implements the `ByteColor` trait for each.
//! - Each method within the trait is implemented to wrap the original value with the appropriate ANSI escape codes.
//! - This approach eliminates repetitive code and ensures consistency across different type implementations.
//!
//! ### Handling Byte Slices and Vectors
//!
//! For byte slices (`&[u8]`) and byte vectors (`Vec<u8>`), the `ByteColor` trait is implemented by first converting the bytes
//! into a `String` using `String::from_utf8_lossy`. This method gracefully handles any invalid UTF-8 sequences, ensuring
//! that the application does not panic at runtime.
//!
//! **Implementation for `&[u8]`:**
//!
//! ```plaintext
//! impl ByteColor for &[u8] {
//!     fn red(&self) -> String {
//!         format!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn green(&self) -> String {
//!         format!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn yellow(&self) -> String {
//!         format!("\x1b[33m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn magenta(&self) -> String {
//!         format!("\x1b[35m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn cyan(&self) -> String {
//!         format!("\x1b[36m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn blue(&self) -> String {
//!         format!("\x1b[34m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn bold(&self) -> String {
//!         format!("\x1b[1m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn underline(&self) -> String {
//!         format!("\x1b[4m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn blink(&self) -> String {
//!         format!("\x1b[5m{}\x1b[0m", String::from_utf8_lossy(self))
//!     }
//!
//!     fn rgb(&self, color: (u8, u8, u8)) -> String {
//!         format!(
//!             "\x1b[38;2;{};{};{}m{}\x1b[0m",
//!             color.0,
//!             color.1,
//!             color.2,
//!             String::from_utf8_lossy(self)
//!         )
//!     }
//!
//!     fn color(&self, color_code: u8) -> String {
//!         format!(
//!             "\x1b[38;5;{}m{}\x1b[0m",
//!             color_code,
//!             String::from_utf8_lossy(self)
//!         )
//!     }
//! }
//! ```
//!
//! **Implementation for `Vec<u8>`:**
//!
//! ```plaintext
//! impl ByteColor for Vec<u8> {
//!     fn red(&self) -> String {
//!         format!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn green(&self) -> String {
//!         format!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn yellow(&self) -> String {
//!         format!("\x1b[33m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn magenta(&self) -> String {
//!         format!("\x1b[35m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn cyan(&self) -> String {
//!         format!("\x1b[36m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn blue(&self) -> String {
//!         format!("\x1b[34m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn bold(&self) -> String {
//!         format!("\x1b[1m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn underline(&self) -> String {
//!         format!("\x1b[4m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn blink(&self) -> String {
//!         format!("\x1b[5m{}\x1b[0m", String::from_utf8_lossy(&self))
//!     }
//!
//!     fn rgb(&self, color: (u8, u8, u8)) -> String {
//!         format!(
//!             "\x1b[38;2;{};{};{}m{}\x1b[0m",
//!             color.0,
//!             color.1,
//!             color.2,
//!             String::from_utf8_lossy(&self)
//!         )
//!     }
//!
//!     fn color(&self, color_code: u8) -> String {
//!         format!(
//!             "\x1b[38;5;{}m{}\x1b[0m",
//!             color_code,
//!             String::from_utf8_lossy(&self)
//!         )
//!     }
//! }
//! ```
//!
//! ## Best Practices
//!
//! - **Trait Visibility:** Ensure that the `ByteColor` trait is in scope wherever its methods are used by importing it with a `use` statement.
//!
//!   ```rust
//!   use bytescolor::ByteColor;
//!   ```
//!
//! - **Consistent Implementation:** Utilize macros to implement the `ByteColor` trait for multiple types, ensuring consistency and reducing code duplication.
//!
//! - **Explicit References:** When calling `ByteColor` methods on primitive types, use references to match the trait implementations.
//!
//!   ```rust
//!   use bytescolor::ByteColor;
//!   let port: u16 = 8080;
//!   println!("{}", (&port).yellow());
//!   ```
//!
//! - **Handle UTF-8 Gracefully:** When working with byte slices or vectors, use `String::from_utf8_lossy` to safely convert bytes to `String`, avoiding potential panics due to invalid UTF-8 sequences.
//!
//! - **Avoid Overlapping Implementations:** Implement the `ByteColor` trait for distinct types to prevent trait coherence issues.
//!
//! ## Caveats and Considerations
//!
//! - **Terminal Compatibility:** ANSI escape codes are widely supported in Unix-like terminals but may not render correctly in all environments, such as some Windows consoles or integrated development environments (IDEs). Always test your application's output in the target terminal environments.
//!
//! - **Performance Overhead:** Applying multiple color and style transformations can introduce performance overhead, especially in applications that perform extensive terminal output operations. Optimize by minimizing unnecessary formatting.
//!
//! - **Readability:** Excessive use of colors and styles can make terminal output cluttered and hard to read. Use them judiciously to highlight important information without overwhelming the user.
//!
//! - **Accessibility:** Consider users with color vision deficiencies. Ensure that your application's color schemes are accessible and provide alternative ways to convey information beyond color alone.
//!
//! ## Extensibility
//!
//! The `ByteColor` trait is designed with extensibility in mind. You can easily extend its functionality by implementing it for additional types
//! or by introducing new methods that cater to specific formatting needs. For instance, you might want to add background color methods or other
//! text styles like italicization.
//!
//! **Example: Adding a Background Color Method**
//!
//! ```plaintext
//! impl ByteColor for &str {
//!     fn background_red(&self) -> String {
//!         format!("\x1b[41m{}\x1b[0m", self)
//!     }
//!
//!     // Implement other background color methods similarly...
//! }
//! ```
//!
//! By following the established pattern, you can enrich the `ByteColor` trait to accommodate a wider range of formatting options, tailoring it
//! to the specific requirements of your application.
//!
//! ## Further Reading
//!
//! - [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code): Comprehensive overview of ANSI codes used for text formatting in terminals.
//! - [Rust Documentation - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html): In-depth guide on Rust traits, their implementation, and best practices.
//! - [Crates.io - Colored](https://crates.io/crates/colored): A popular crate for colorizing terminal output in Rust.
//! - [Crates.io - ANSI Term](https://crates.io/crates/ansi_term): Another crate offering advanced ANSI terminal styling capabilities.
//!
//! ## License
//!
//! This project is licensed under the Apache-2.0.
pub trait ByteColor {
    /// Applies red color to the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let number: u32 = 42;
    /// println!("{}", number.red()); // Displays "42" in red
    /// ```
    fn red(&self) -> String;

    /// Applies green color to the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let message: &str = "Success!";
    /// println!("{}", message.green()); // Displays "Success!" in green
    /// ```
    fn green(&self) -> String;

    /// Applies yellow color to the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let warning: &str = "Warning!";
    /// println!("{}", warning.yellow()); // Displays "Warning!" in yellow
    /// ```
    fn yellow(&self) -> String;

    /// Applies magenta color to the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let info: &str = "Information";
    /// println!("{}", info.magenta()); // Displays "Information" in magenta
    /// ```
    fn magenta(&self) -> String;

    /// Applies cyan color to the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let data: &str = "Cyan Data";
    /// println!("{}", data.cyan()); // Displays "Cyan Data" in cyan
    /// ```
    fn cyan(&self) -> String;

    /// Applies blue color to the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let message: &str = "Blue Message";
    /// println!("{}", message.blue()); // Displays "Blue Message" in blue
    /// ```
    fn blue(&self) -> String;

    /// Makes the text bold.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let emphasized: &str = "Important!";
    /// println!("{}", emphasized.bold()); // Displays "Important!" in bold
    /// ```
    fn bold(&self) -> String;

    /// Underlines the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let underlined: &str = "Underlined Text";
    /// println!("{}", underlined.underline()); // Displays "Underlined Text" underlined
    /// ```
    fn underline(&self) -> String;

    /// Makes the text blink.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let blinking: &str = "Blinking Text";
    /// println!("{}", blinking.blink()); // Displays "Blinking Text" with a blinking effect
    /// ```
    fn blink(&self) -> String;

    /// Applies a custom RGB color to the text.
    ///
    /// # Parameters
    ///
    /// - `rgb`: A tuple representing the red, green, and blue components of the color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let custom_rgb: &str = "RGB Colored Text";
    /// println!("{}", custom_rgb.rgb((70, 130, 180))); // Displays the text in Steel Blue
    /// ```
    fn rgb(&self, rgb: (u8, u8, u8)) -> String;

    /// Applies a custom 256-color palette color to the text using a color code.
    ///
    /// # Parameters
    ///
    /// - `code`: An ANSI color code ranging from 0 to 255.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytescolor::ByteColor;
    ///
    /// let custom_color: &str = "Custom Color";
    /// println!("{}", custom_color.color(202)); // Displays "Custom Color" in a specific shade of orange
    /// ```
    fn color(&self, code: u8) -> String;
}

macro_rules! impl_colorize_for_primitive {
    ($($t:ty),*) => {
        $(
            impl ByteColor for $t {
                fn red(&self) -> String {
                    format!("\x1b[31m{}\x1b[0m", self)
                }

                fn green(&self) -> String {
                    format!("\x1b[32m{}\x1b[0m", self)
                }

                fn yellow(&self) -> String {
                    format!("\x1b[33m{}\x1b[0m", self)
                }

                fn magenta(&self) -> String {
                    format!("\x1b[35m{}\x1b[0m", self)
                }

                fn cyan(&self) -> String {
                    format!("\x1b[36m{}\x1b[0m", self)
                }

                fn blue(&self) -> String {
                    format!("\x1b[34m{}\x1b[0m", self)
                }

                fn bold(&self) -> String {
                    format!("\x1b[1m{}\x1b[0m", self)
                }

                fn underline(&self) -> String {
                    format!("\x1b[4m{}\x1b[0m", self)
                }

                fn blink(&self) -> String {
                    format!("\x1b[5m{}\x1b[0m", self)
                }

                fn rgb(&self, color: (u8, u8, u8)) -> String {
                    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", color.0, color.1, color.2, self)
                }

                fn color(&self, color_code: u8) -> String {
                    format!("\x1b[38;5;{}m{}\x1b[0m", color_code, self)
                }
            }
        )*
    };
}

// Apply the macro to primitive types
impl_colorize_for_primitive!(u8, u16, u32, u64, i8, i16, i32, i64, usize);

// Implement ByteColor for &str
impl ByteColor for &str {
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
    }

    fn magenta(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self)
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[0m", self)
    }

    fn rgb(&self, rgb: (u8, u8, u8)) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", rgb.0, rgb.1, rgb.2, self)
    }

    fn color(&self, code: u8) -> String {
        format!("\x1b[38;5;{}m{}\x1b[0m", code, self)
    }
}

// Implement ByteColor for String
impl ByteColor for String {
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
    }

    fn magenta(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self)
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[0m", self)
    }

    fn rgb(&self, rgb: (u8, u8, u8)) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", rgb.0, rgb.1, rgb.2, self)
    }

    fn color(&self, code: u8) -> String {
        format!("\x1b[38;5;{}m{}\x1b[0m", code, self)
    }
}

// Implement ByteColor for &[u8]
impl ByteColor for &[u8] {
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn magenta(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[0m", String::from_utf8_lossy(self))
    }

    fn rgb(&self, rgb: (u8, u8, u8)) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            rgb.0,
            rgb.1,
            rgb.2,
            String::from_utf8_lossy(self)
        )
    }

    fn color(&self, code: u8) -> String {
        format!(
            "\x1b[38;5;{}m{}\x1b[0m",
            code,
            String::from_utf8_lossy(self)
        )
    }
}

// Implement ByteColor for Vec<u8>
impl ByteColor for Vec<u8> {
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn magenta(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn blink(&self) -> String {
        format!("\x1b[5m{}\x1b[0m", String::from_utf8_lossy(&self))
    }

    fn rgb(&self, rgb: (u8, u8, u8)) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            rgb.0,
            rgb.1,
            rgb.2,
            String::from_utf8_lossy(&self)
        )
    }

    fn color(&self, code: u8) -> String {
        format!(
            "\x1b[38;5;{}m{}\x1b[0m",
            code,
            String::from_utf8_lossy(&self)
        )
    }
}

// Implement ByteColor for &[u8; N]
impl<const N: usize> ByteColor for &[u8; N] {
    fn red(&self) -> String {
        self.as_ref().red()
    }

    fn green(&self) -> String {
        self.as_ref().green()
    }

    fn yellow(&self) -> String {
        self.as_ref().yellow()
    }

    fn magenta(&self) -> String {
        self.as_ref().magenta()
    }

    fn cyan(&self) -> String {
        self.as_ref().cyan()
    }

    fn blue(&self) -> String {
        self.as_ref().blue()
    }

    fn bold(&self) -> String {
        self.as_ref().bold()
    }

    fn underline(&self) -> String {
        self.as_ref().underline()
    }

    fn blink(&self) -> String {
        self.as_ref().blink()
    }

    fn rgb(&self, rgb: (u8, u8, u8)) -> String {
        self.as_ref().rgb(rgb)
    }

    fn color(&self, code: u8) -> String {
        self.as_ref().color(code)
    }
}
