mod layout;
pub use layout::{headings::*, Footer, Header, Page, Paragraph};

mod navigation;
use navigation::Navigation;

mod social_media_icons;

mod body;
pub use body::Body;

mod input_toggle;
pub use input_toggle::InputToggle;

pub mod util;
pub use util::InlineAnchor;

pub mod web_project;
pub use web_project::{WebProject, WebProjectAnalytics, WebProjectProps};

mod cloudinary_image;
pub use cloudinary_image::CloudinaryImage;

mod error_message;
pub use error_message::ErrorMessage;

mod success_message;
pub use success_message::SuccessMessage;
