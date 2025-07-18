//! GUI styling and theming system
//!
//! The style module provides a flexible theming system for Kolibri GUIs. It controls the visual
//! appearance of the UI, including colors, spacing, fonts, and other visual aspects of the
//! interface. This allows compatibility and abstraction over [embedded_graphics::pixelcolor]
//! color types, making it easy to switch between color depths or display technologies.
//! Several predefined themes are included for [Rgb565] displays (e.g. ILI9341).
//!
//! # Examples
//!
//! Using a predefined theme:
//! ```no_run
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics::primitives::Rectangle;
//! # use embedded_iconoir::prelude::*;
//! # use embedded_iconoir::size12px;
//! # use kolibri_embedded_gui::ui::*;
//! # use kolibri_embedded_gui::label::*;
//! # use kolibri_embedded_gui::smartstate::*;
//! # let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
//! # let output_settings = OutputSettingsBuilder::new().build();
//! # let mut window = Window::new("Kolibri Example", &output_settings);
//! use kolibri_embedded_gui::style::medsize_rgb565_style;
//! use kolibri_embedded_gui::ui::Ui;
//!
//! // Create UI with dark theme
//! let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
//! ```
//!
//! Switching themes at runtime:
//! ```no_run
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics::primitives::Rectangle;
//! # use embedded_iconoir::prelude::*;
//! # use embedded_iconoir::size12px;
//! # use kolibri_embedded_gui::ui::*;
//! # use kolibri_embedded_gui::label::*;
//! # use kolibri_embedded_gui::smartstate::*;
//! # let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
//! # let output_settings = OutputSettingsBuilder::new().build();
//! # let mut window = Window::new("Kolibri Example", &output_settings);
//! use kolibri_embedded_gui::style::{medsize_rgb565_style, medsize_light_rgb565_style};
//!
//! let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
//! // Later...
//! *ui.style_mut() = medsize_light_rgb565_style(); // Switch to light theme
//! ```

use embedded_graphics::mono_font::{self, MonoFont};
use embedded_graphics::pixelcolor::{PixelColor, Rgb565, Rgb888};
use embedded_graphics::prelude::*;

/// Controls spacing between UI elements.
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    /// Space between adjacent items in the UI
    pub item_spacing: Size,
    /// Internal padding within buttons
    pub button_padding: Size,
    /// Padding around the border of a widget (e.g. a checkbox)
    pub default_padding: Size,
    /// Padding inside window borders
    pub window_border_padding: Size,
}

// an interactive widget can have a context that determines the style
#[derive(Debug)]
pub enum WidgetContext {
    Normal,
    Primary,
    Secondary,
}

// a StateStyle should be specified for each WidgetState for each Context
#[derive(Debug, Clone, Copy)]
pub struct WidgetStyle<COL: PixelColor> {
    pub border_width: u32,
    pub border_color: COL,
    pub background_color: COL,
    pub foreground_color: COL,
}
#[derive(Debug, Clone, Copy)]
pub struct WidgetContextStyle<COL: PixelColor> {
    pub normal: WidgetStyle<COL>,
    pub hover:  WidgetStyle<COL>,
    pub active: WidgetStyle<COL>,
    pub disabled: WidgetStyle<COL>,
}

#[derive(Debug, Clone, Copy)]
pub struct Style<COL: PixelColor> {
    /// Background color for the entire UI
    pub background_color: COL,
    /// Default font used for text rendering
    pub default_font: MonoFont<'static>,
    /// Color used for text
    pub text_color: COL,
    pub normal_widget: WidgetContextStyle<COL>,
    pub primary_widget: WidgetContextStyle<COL>,
    pub secondary_widget: WidgetContextStyle<COL>,
    /// Default height for widgets like buttons
    pub default_widget_height: u32,
    /// Spacing configuration for UI elements
    pub spacing: Spacing,
    /// radius for button corners
    pub button_corner_radius: u32,
}

/*
/// Debug-friendly dark theme with visible borders for development.
///
/// This theme uses high-contrast colors and visible borders to make UI layout
/// and component boundaries clear during development.
pub fn medsize_rgb565_debug_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::BLACK,
        text_color: Rgb565::WHITE,
        primary : ContextStyle { 
            normal: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::RED, 
                background_color: Rgb565::CSS_GRAY, 
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::new(0x1, 0x2, 0x1), 
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::CYAN, 
                foreground_color: Rgb565::WHITE, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },
        secondary : ContextStyle { 
            normal: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::new(0x1, 0x2, 0x1), 
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::CYAN, 
                foreground_color: Rgb565::WHITE, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },
        default_widget_height: 16,
        default_font: mono_font::iso_8859_10::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(2, 2),
            default_padding: Size::new(3, 3),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 0,
    }
}

/// Dark theme for RGB565 displays.
///
/// Features a dark gray background with cyan accents and white text.
pub fn medsize_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::new(0x4, 0x8, 0x4), // pretty dark gray
        text_color: Rgb565::WHITE,

        primary : ContextStyle { 
            normal: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE,
                background_color: Rgb565::new(0x2, 0x4, 0x2), // darker gray
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::new(0x1, 0x2, 0x1),
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::new(0x2, 0x4, 0x2), // darker gray
                foreground_color: Rgb565::CSS_DARK_CYAN, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::new(0x4, 0x8, 0x4), // pretty dark gray
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },
        secondary : ContextStyle { 
            normal: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::new(0x1, 0x2, 0x1),
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::CYAN, 
                foreground_color: Rgb565::CSS_DARK_CYAN, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },
        default_widget_height: 16,
        default_font: mono_font::iso_8859_10::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 3,
    }
}
     */
/*
/// Light theme for RGB565 displays.
///
/// Features a white background with orange accents and black text.
pub fn medsize_light_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_WHITE,
        item_background_color: Rgb565::CSS_NAVAJO_WHITE,
        highlight_item_background_color: Rgb565::CSS_GAINSBORO,
        border_color: Rgb565::CSS_WHITE,
        highlight_border_color: Rgb565::CSS_BLACK,
        primary_color: Rgb565::CSS_DARK_ORANGE,
        secondary_color: Rgb565::YELLOW,
        icon_color: Rgb565::CSS_BLACK,
        text_color: Rgb565::CSS_BLACK,
        default_widget_height: 16,
        border_width: 0,
        highlight_border_width: 1,
        default_font: mono_font::iso_8859_10::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 0,
    }
}

/// Pink theme for RGB565 displays.
///
/// Features a peach background with pink accents and black text.
pub fn medsize_sakura_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_PEACH_PUFF,
        text_color: Rgb565::CSS_BLACK,
        
        primary : ContextStyle { 
            normal: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE,
                background_color: Rgb565::CSS_LIGHT_PINK,
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::CSS_BLACK, 
                background_color: Rgb565::CSS_HOT_PINK,
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::CSS_HOT_PINK,
                foreground_color: Rgb565::CSS_DARK_CYAN, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::CSS_PEACH_PUFF,
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },
        secondary : ContextStyle { 
            normal: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::CSS_BLACK, 
                background_color: Rgb565::CSS_HOT_PINK,
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::CSS_HOT_PINK, 
                foreground_color: Rgb565::CSS_DARK_CYAN, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::BLACK, 
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },        
        
        default_widget_height: 16,
        default_font: mono_font::ascii::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 5,
    }
}
/*
/// Blue theme for RGB565 displays.
///
/// Features a midnight blue background with violet accents and white text.
pub fn medsize_blue_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_MIDNIGHT_BLUE,
        item_background_color: Rgb565::CSS_BLUE,
        highlight_item_background_color: Rgb565::CSS_BLUE_VIOLET,
        border_color: Rgb565::CSS_WHITE,
        highlight_border_color: Rgb565::CSS_WHITE,
        primary_color: Rgb565::CSS_PALE_VIOLET_RED,
        secondary_color: Rgb565::YELLOW,
        icon_color: Rgb565::CSS_WHITE,
        text_color: Rgb565::CSS_WHITE,
        default_widget_height: 16,
        border_width: 0,
        highlight_border_width: 1,
        default_font: mono_font::iso_8859_10::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 0,
    }
}

/// Retro CRT monitor theme for RGB565 displays.
///
/// Features a black background with green text and borders, reminiscent of early CRT monitors.
pub fn medsize_crt_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_BLACK,
        item_background_color: Rgb565::CSS_BLACK,
        highlight_item_background_color: Rgb565::CSS_BLACK,
        border_color: Rgb565::CSS_GREEN,
        highlight_border_color: Rgb565::CSS_GREEN,
        primary_color: Rgb565::CSS_GREEN,
        secondary_color: Rgb565::YELLOW,
        icon_color: Rgb565::CSS_GREEN,
        text_color: Rgb565::CSS_GREEN,
        default_widget_height: 16,
        border_width: 1,
        highlight_border_width: 3,
        default_font: mono_font::iso_8859_10::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 0,
    }
}

/// Minimalist black and white theme for RGB565 displays.
///
/// Features a white background with black borders and text, suitable for high contrast displays or e-ink screens.
pub fn medsize_retro_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_WHITE,
        item_background_color: Rgb565::CSS_WHITE,
        highlight_item_background_color: Rgb565::CSS_WHITE,
        border_color: Rgb565::CSS_BLACK,
        highlight_border_color: Rgb565::CSS_BLACK,
        primary_color: Rgb565::CSS_BLACK,
        secondary_color: Rgb565::YELLOW,
        icon_color: Rgb565::CSS_BLACK,
        text_color: Rgb565::CSS_BLACK,
        default_widget_height: 16,
        border_width: 1,
        highlight_border_width: 1,
        default_font: mono_font::ascii::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 0,
    }
}
 */
/// Defines the visual appearance of a Kolibri UI.
///
/// The [Style] struct controls all visual aspects of the UI, including colors,
/// spacing, fonts, and dimensions. It is generic over the color type to support
/// different color depths (e.g., RGB565, grayscale, or monochrome).
///
/// # Examples
///
/// Creating a custom style:
/// ```rust
/// use embedded_graphics::pixelcolor::Rgb565;
/// use embedded_graphics::mono_font;
/// use kolibri_embedded_gui::style::{Style, Spacing};
/// use embedded_graphics::prelude::*;
///
/// let custom_style = Style {
///     background_color: Rgb565::BLACK,
///     text_color: Rgb565::WHITE,
///     primary_color: Rgb565::BLUE,
///     spacing: Spacing {
///         item_spacing: Size::new(10, 5),
///         button_padding: Size::new(4, 4),
///         default_padding: Size::new(2, 2),
///         window_border_padding: Size::new(3, 3),
///     },
///     default_font: mono_font::ascii::FONT_6X13,
///     border_color: Rgb565::BLACK,
///     border_width: 1,
///     default_widget_height: 16,
///     icon_color: Rgb565::BLACK,
///     secondary_color: Rgb565::YELLOW,
///     highlight_border_color: Rgb565::WHITE,
///     highlight_border_width: 2,
///     highlight_item_background_color: Rgb565::BLUE,
///     item_background_color: Rgb565::BLACK,
///     button_corner_radius: 5,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Style<COL: PixelColor> {
    /// Background color for the entire UI
    pub background_color: COL,
    /// Color used for borders around widgets
    pub border_color: COL,
    /// Primary accent color for interactive elements
    pub primary_color: COL,
    /// Secondary accent color for additional highlighting
    pub secondary_color: COL,
    /// Color used for icons
    pub icon_color: COL,
    /// Default height for widgets like buttons
    pub default_widget_height: u32,
    /// Width of borders around widgets
    pub border_width: u32,
    /// Default font used for text rendering
    pub default_font: MonoFont<'static>,
    /// Spacing configuration for UI elements
    pub spacing: Spacing,
    /// Background color for items like buttons
    pub item_background_color: COL,
    /// Background color for highlighted items
    pub highlight_item_background_color: COL,
    /// Border color for highlighted elements
    pub highlight_border_color: COL,
    /// Border width for highlighted elements
    pub highlight_border_width: u32,
    /// Color used for text
    pub text_color: COL,
    /// radius for button corners
    pub button_corner_radius: u32,
}
*/

/// Bootstrap-inspired theme for RGB565 displays.
///
/// Features a dark background with white text.
// defined as from(Rgb888) to allow direct comparison with standard web/rgb colors and color pickers
pub fn medsize_bootstrap_rgb565_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_BLACK,
        text_color: Rgb565::WHITE,
        normal_widget : WidgetContextStyle { 
            normal: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE,
                background_color: Rgb565::CSS_BLACK,
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::CSS_LIGHT_GRAY,
                foreground_color: Rgb565::CSS_BLACK,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::WHITE, 
                background_color: Rgb565::WHITE,
                foreground_color: Rgb565::CSS_BLACK, 
            }, 
            disabled: WidgetStyle { 
                border_width: 1, 
                border_color: Rgb565::CSS_DARK_GRAY, 
                background_color: Rgb565::CSS_BLACK,
                foreground_color: Rgb565::CSS_DARK_GRAY, 
            } 
        },        
        primary_widget : WidgetContextStyle { 
            normal: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(13,110,253)), // rgb(13,110,253)
                background_color: Rgb565::from(Rgb888::new(13,110,253)), // rgb(13,110,253)
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(0x0b,0x5e,0xd7)), // #0B5ED7
                background_color: Rgb565::from(Rgb888::new(0x0b,0x5e,0xd7)), // rgba(11, 94, 215, 1)
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(10,88,202)), // rgb(10,88,202)
                background_color: Rgb565::from(Rgb888::new(10,88,202)), // rgb(10,88,202)
                foreground_color: Rgb565::WHITE, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(0x13, 0x54, 0xb3)), // rgba(19, 84, 179, 1)
                background_color: Rgb565::from(Rgb888::new(0x13, 0x54, 0xb3)), // rgba(19, 84, 179, 1)
                foreground_color: Rgb565::CSS_LIGHT_GRAY, 
            } 
        },
        secondary_widget : WidgetContextStyle { 
            normal: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(108,117,125)), // rgb(108,117,125)
                background_color: Rgb565::from(Rgb888::new(108,117,125)), // rgb(108,117,125)
                foreground_color: Rgb565::WHITE, 
            },
            hover: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(92, 99,106)), //  rgb(92, 99, 106)
                background_color:  Rgb565::from(Rgb888::new(92, 99,106)), //  rgb(92, 99, 106)
                foreground_color: Rgb565::WHITE,  
            }, 
            active: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(0x0a, 0x58, 0xca)),// rgb(76, 81, 91)
                background_color: Rgb565::from(Rgb888::new(0x0a, 0x58, 0xca)),// rgb(76, 81, 91)
                foreground_color: Rgb565::WHITE, 
            }, 
            disabled: WidgetStyle { 
                border_width: 0, 
                border_color: Rgb565::from(Rgb888::new(81,89, 95)), // rgb(81, 89, 95)
                background_color: Rgb565::from(Rgb888::new(81,89, 95)), // rgb(81, 89, 95)
                foreground_color: Rgb565::from(Rgb888::new(177,179,180)), // rgb(177, 179, 180)
            } 
        },        
        
        default_widget_height: 16,
        default_font: mono_font::ascii::FONT_9X15,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
        button_corner_radius: 5,
    }
}