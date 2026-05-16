pub mod base;
pub mod factory;
pub mod txt;
pub mod md;
pub mod stdin;

#[cfg(feature = "html")]
pub mod html;
#[cfg(feature = "docx")]
pub mod docx;
#[cfg(feature = "pdf")]
pub mod pdf;
#[cfg(feature = "epub")]
pub mod epub;
#[cfg(feature = "fb2")]
pub mod fb2;
#[cfg(feature = "rtf")]
pub mod rtf;
