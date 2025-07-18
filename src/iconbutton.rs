//! # IconButton Widget
//!
//! The [IconButton] widget combines an icon with button interaction capabilities.
//! It provides a compact way to create clickable icons with optional subtitles,
//! supporting all the interaction states of standard buttons.
//!
//! ## Core Features
//!
//! - Combines icon display with button interaction (click, hover, press states)
//! - Optional subtitle/label text below the icon
//! - Visual feedback via color changes for different interaction states
//! - Integration with Kolibri's theming system
//! - Support for the smartstate system for efficient redrawing
//! - option to specify minimum width
//!
//! ## Usage
//!
//! ```no_run
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
//! # use kolibri_embedded_gui::style::medsize_rgb565_style;
//! # use kolibri_embedded_gui::ui::Ui;
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
//! # let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
//! # use kolibri_embedded_gui::iconbutton::IconButton;
//! # use embedded_iconoir::size12px::actions::AddCircle;
//! // Basic icon button
//! ui.add(IconButton::new(size12px::actions::AddCircle));
//!
//! // Icon button with subtitle
//! ui.add(IconButton::new(size12px::actions::AddCircle).label("Settings"));
//!
//! // Using with the type system instead of passing an icon instance
//! ui.add(IconButton::<size12px::actions::AddCircle>::new_from_type());
//!
//! // Using smartstate for efficient redrawing
//! let mut smartstateProvider = SmartstateProvider::<20>::new();
//! ui.add(IconButton::new(size12px::actions::AddCircle).smartstate(smartstateProvider.nxt()));
//!
//! // Handling button clicks
//! if ui.add(IconButton::new(size12px::actions::AddCircle)).clicked() {
//!     // Handle the click action
//! }
//! ```
//!
//! ## Implementation Details
//!
//! The [IconButton] widget uses different visual styles based on interaction state:
//! - Normal: Standard background and border colors
//! - Hover: Highlighted background and border for visual feedback
//! - Pressed/Active: Primary color background with highlighted border
//!
use crate::smartstate::{Container, Smartstate};
use crate::style::{WidgetContext};
use crate::ui::{GuiResult, Interaction, Response, Ui, Widget};
use core::cmp::max;
use core::marker::PhantomData;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle};
use embedded_graphics::text::{Alignment, Baseline, Text};
use embedded_iconoir::prelude::{IconoirIcon, IconoirNewIcon};

/// A button widget that displays an icon with optional text label.
///
/// [IconButton] combines the visual display of an icon with interactive button
/// behavior. It changes appearance based on user interaction (normal, hover, pressed)
/// and can optionally display a text label underneath the icon.
pub struct IconButton<'a, ICON: IconoirIcon> {
    icon: PhantomData<ICON>,
    label: Option<&'a str>,
    smartstate: Container<'a, Smartstate>,
    min_width : u32,
    is_enabled: bool,  // when not enabled does not respond to interaction
    is_modified: bool, // set when min_width or enabled is changed
    context: WidgetContext,
}

impl<'a, ICON: IconoirIcon> IconButton<'a, ICON> {
    /// Creates a new [IconButton] from an [IconoirIcon] instance.
    ///
    /// The icon color from the icon instance will be ignored, as the widget
    /// will use the icon color from the current UI style.
    ///
    /// To see all icons you can use, look at [embedded_iconoir::size12px].
    /// All other icon resolutions (from [embedded_iconoir::size12px] to [embedded_iconoir::size144px]) are available.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
    /// # use kolibri_embedded_gui::style::medsize_rgb565_style;
    /// # use kolibri_embedded_gui::ui::Ui;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_iconoir::prelude::*;
    /// # use embedded_iconoir::size12px;
    /// # use kolibri_embedded_gui::ui::*;
    /// # use kolibri_embedded_gui::label::*;
    /// # use kolibri_embedded_gui::smartstate::*;
    /// # let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    /// # let output_settings = OutputSettingsBuilder::new().build();
    /// # let mut window = Window::new("Kolibri Example", &output_settings);
    /// # let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
    /// # use kolibri_embedded_gui::iconbutton::IconButton;
    /// use embedded_iconoir::size24px;
    /// ui.add(IconButton::new(size24px::actions::AddCircle));
    /// ```
    pub fn new(_icon: ICON) -> Self {
        Self {
            icon: PhantomData,
            smartstate: Container::empty(),
            label: None,
            min_width : 0u32,
            is_enabled: true,
            is_modified: false,
            context: WidgetContext::Normal,
        }
    }

    /// Adds a text label/subtitle below the icon.
    ///
    /// The label text will be centered below the icon and sized according
    /// to the current UI style font settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
    /// # use kolibri_embedded_gui::style::medsize_rgb565_style;
    /// # use kolibri_embedded_gui::ui::Ui;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_iconoir::prelude::*;
    /// # use embedded_iconoir::size12px;
    /// # use kolibri_embedded_gui::ui::*;
    /// # use kolibri_embedded_gui::label::*;
    /// # use kolibri_embedded_gui::smartstate::*;
    /// # let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    /// # let output_settings = OutputSettingsBuilder::new().build();
    /// # let mut window = Window::new("Kolibri Example", &output_settings);
    /// # let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
    /// # use kolibri_embedded_gui::iconbutton::IconButton;
    /// use embedded_iconoir::size24px;
    /// ui.add(IconButton::new(size24px::actions::AddCircle).label("Add"));
    /// ```
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Creates a new [IconButton] using just the icon's type.
    ///
    /// This is a convenience method that allows creating an icon button without
    /// instantiating the icon object first.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
    /// # use kolibri_embedded_gui::style::medsize_rgb565_style;
    /// # use kolibri_embedded_gui::ui::Ui;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_iconoir::prelude::*;
    /// # use embedded_iconoir::size12px;
    /// # use kolibri_embedded_gui::ui::*;
    /// # use kolibri_embedded_gui::label::*;
    /// # use kolibri_embedded_gui::smartstate::*;
    /// # let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    /// # let output_settings = OutputSettingsBuilder::new().build();
    /// # let mut window = Window::new("Kolibri Example", &output_settings);
    /// # let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
    /// # use kolibri_embedded_gui::iconbutton::IconButton;
    /// use embedded_iconoir::size24px;
    /// ui.add(IconButton::<size24px::actions::AddCircle>::new_from_type());
    /// ```
    pub fn new_from_type() -> Self {
        Self {
            icon: PhantomData,
            smartstate: Container::empty(),
            label: None,
            min_width: 0_u32,
            is_enabled: true,
            is_modified: false,
            context: WidgetContext::Normal,
        }
    }

    /// Attaches a [Smartstate] to this widget for incremental redrawing.
    ///
    /// When a smartstate is attached, the widget will only redraw when its
    /// state changes, improving performance for stationary UI elements.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// # use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};
    /// # use kolibri_embedded_gui::style::medsize_rgb565_style;
    /// # use kolibri_embedded_gui::ui::Ui;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_iconoir::prelude::*;
    /// # use embedded_iconoir::size12px;
    /// # use kolibri_embedded_gui::ui::*;
    /// # use kolibri_embedded_gui::label::*;
    /// # use kolibri_embedded_gui::smartstate::*;
    /// # let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    /// # let output_settings = OutputSettingsBuilder::new().build();
    /// # let mut window = Window::new("Kolibri Example", &output_settings);
    /// # let mut ui = Ui::new_fullscreen(&mut display, medsize_rgb565_style());
    /// # use kolibri_embedded_gui::iconbutton::IconButton;
    /// let mut my_smartstate = Smartstate::empty();
    /// ui.add(IconButton::new(size12px::actions::AddCircle).smartstate(&mut my_smartstate));
    /// ```
    ///
    /// Returns `self` for method chaining.
    pub fn smartstate(mut self, smartstate: &'a mut Smartstate) -> Self {
        self.smartstate.set(smartstate);
        self
    }

   /// Specifies minimum width to override automatic width to fit contents. 
    ///
    /// # Arguments
    /// * `width` - The minimum width.  0 will result in automatic sizing
    ///
    /// # Returns
    /// Self with minimum width configured
    pub fn expand_width(mut self, width: u32) -> Self {
        self.min_width = width;
        self.is_modified = true;
        self
    }

    /// Enables or disables the widget - will not respond to interaction when not enabled
    ///
    /// # Arguments
    /// * `enabled` - if the button should be enabled (true) or disabled(false)
    /// 
    /// # Returns
    /// Self with is_enabled set
    pub fn enable(mut self, enabled: &bool) -> Self {
        self.is_modified = true;
        self.is_enabled = *enabled;
        self
    }
    
    /// Specifies the context for the widget to determine how it is styled
    ///
    /// # Arguments
    /// * `context` - Context::Normal, Context::Primary, Context::Secondary
    /// 
    /// # Returns
    /// Self with context set
    pub fn context(mut self, context: WidgetContext) -> Self {
        self.is_modified = true;
        self.context = context;
        self
    }
}

impl<COL: PixelColor, ICON: IconoirIcon> Widget<COL> for IconButton<'_, ICON> {
    /// Draws the icon button within the UI.
    ///
    /// This method:
    /// 1. Calculates the size based on icon and optional label, increasing width to match minimum if necessary
    /// 2. Allocates space for the widget
    /// 3. Positions the icon and optional label
    /// 4. Detects interactions (hover, click, press)
    /// 5. Manages visual appearance based on interaction state
    /// 6. Updates the smartstate and draws when necessary
    /// 7. Returns a response that includes click information
    fn draw<DRAW: DrawTarget<Color = COL>>(
        &mut self,
        ui: &mut Ui<DRAW, COL>,
    ) -> GuiResult<Response> {
        
        let mut fg_color = ui.style().normal_widget.normal.foreground_color;
        // get size
        let mut icon = ICON::new(fg_color);

        let padding = ui.style().spacing.button_padding;
        let border = ui.style().normal_widget.normal.border_width;

        let mut min_height = icon.bounding_box().size.height + 2 * padding.height + 2 * border;

        let mut width = min_height;

        let font = ui.style().default_font;

        let mut text = if let Some(label) = self.label {
            let mut text = Text::new(
                label,
                Point::new(0, 0),
                MonoTextStyle::new(&font, fg_color),
            );
            text.text_style.alignment = Alignment::Center;
            text.text_style.baseline = Baseline::Top;
            min_height += padding.height + text.bounding_box().size.height;
            width = width.max(text.bounding_box().size.width + 2 * padding.width + 2 * border);
            Some(text)
        } else {
            None
        };
        let height = max(
            max(ui.style().default_widget_height, ui.get_row_height()),
            min_height,
        );

        if width < self.min_width {
            width = self.min_width;
        }

        let size = Size::new(width, height);

        /*
        let icon = match size.width - 2 * padding.width {
            0..=17 => 12,
            18..=24 => 18,
            24..=32 => 24,
            _ => 32,
        };
         */

        // allocate space
        let iresponse = ui.allocate_space(Size::new(size.width, max(size.height, height)))?;

        // translate icon
        let size = icon.bounding_box();

        // center icon
        let center_offset = iresponse.area.top_left
            + Point::new(
                ((iresponse.area.size.width - size.size.width) / 2) as i32,
                ((iresponse.area.size.height
                    - size.size.height
                    - text
                        .map(|t| t.bounding_box().size.height + padding.height)
                        .unwrap_or(0))
                    / 2) as i32,
            );


        // center text (if it exists)
        if let Some(text) = text.as_mut() {
            let center_offset = iresponse.area.top_left
                + Point::new(
                    (iresponse.area.size.width / 2) as i32,
                    (iresponse.area.size.height
                        - text.bounding_box().size.height
                        - padding.height
                        - border) as i32,
                );
            text.translate_mut(center_offset);
        }

        // check for click
        let click = matches!(iresponse.interaction, Interaction::Release(_));
        let down = matches!(
            iresponse.interaction,
            Interaction::Click(_) | Interaction::Drag(_)
        );

        // styles and smartstate
        let prevstate = self.smartstate.clone_inner();
        let rect_style: PrimitiveStyle<COL>;
        let context_style = match self.context {
            WidgetContext::Normal => ui.style().normal_widget,
            WidgetContext::Primary => ui.style().primary_widget.unwrap_or_else(|| ui.style().normal_widget),
            WidgetContext::Secondary => ui.style().secondary_widget.unwrap_or_else(|| ui.style().normal_widget),
        };

        if self.is_enabled {
            rect_style = match iresponse.interaction {
                Interaction::None => {
                    if self.is_modified {
                        self.smartstate.modify(|st| *st = Smartstate::state(1));
                    } else {
                        self.smartstate.modify(|st| *st = Smartstate::state(2));
                    }

                    PrimitiveStyleBuilder::new()
                        .stroke_color(context_style.normal.border_color)
                        .stroke_width(context_style.normal.border_width)
                        .fill_color(context_style.normal.background_color)
                        .build()
                }
                Interaction::Hover(_) => {
                    if self.is_modified {
                        self.smartstate.modify(|st| *st = Smartstate::state(3));
                    } else {
                        self.smartstate.modify(|st| *st = Smartstate::state(4));
                    }
                    PrimitiveStyleBuilder::new()
                        .stroke_color(context_style.hover.border_color)
                        .stroke_width(context_style.hover.border_width)
                        .fill_color(context_style.hover.background_color)
                        .build()
                }

                _ => {
                    if self.is_modified {
                        self.smartstate.modify(|st| *st = Smartstate::state(5));
                    } else {
                        self.smartstate.modify(|st| *st = Smartstate::state(6));
                    }

                    PrimitiveStyleBuilder::new()
                        .stroke_color(context_style.active.border_color)
                        .stroke_width(context_style.active.border_width)
                        .fill_color(context_style.active.background_color)
                        .build()
                }
            };

            match iresponse.interaction {
                Interaction::None => {
                    fg_color = context_style.normal.foreground_color;
                }
                Interaction::Hover(_) => {
                    fg_color = context_style.hover.foreground_color;
                }
                _ => {
                    fg_color = context_style.active.foreground_color;
                }
            };

        } else {
            if self.is_modified {
                self.smartstate.modify(|st| *st = Smartstate::state(7));
            } else {
                self.smartstate.modify(|st| *st = Smartstate::state(8));
            }

            rect_style = PrimitiveStyleBuilder::new()
                .stroke_color(context_style.disabled.border_color)
                .stroke_width(context_style.disabled.border_width)
                .fill_color(context_style.disabled.background_color)
                .build();
            fg_color = context_style.disabled.foreground_color;
        }
        icon.set_color(fg_color);
        let icon_img = Image::new(&icon, center_offset);

        if let Some(text) = text.as_mut() {
            text.character_style.text_color = Some(fg_color);
        }


        if !self.smartstate.eq_option(&prevstate) {
            ui.start_drawing(&iresponse.area);

            ui.draw(
                &RoundedRectangle::with_equal_corners(
                    Rectangle::new(iresponse.area.top_left, iresponse.area.size),
                    Size::new(ui.style().button_corner_radius,ui.style().button_corner_radius)
                )
                .into_styled(rect_style),
            )
            .ok();
            ui.draw(&icon_img).ok();
            if let Some(text) = text.as_mut() {
                ui.draw(text).unwrap();
            }

            ui.finalize()?;
        }

        if self.is_enabled {
            Ok(Response::new(iresponse).set_clicked(click).set_down(down))
        } else {
            Ok(Response::new(iresponse).set_clicked(false).set_down(false))
        }
    }
}

// Implement common traits for IconButton
impl<ICON: IconoirIcon> core::fmt::Debug for IconButton<'_, ICON> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IconButton")
            .field("type", &core::any::type_name::<ICON>())
            .field("label", &self.label)
            .field("smartstate", &"<smartstate>")
            .finish()
    }
}
