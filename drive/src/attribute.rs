use darling::FromAttributes;
use syn::Attribute;

use crate::error::AttributeParseError;

/// Represents the different types of attributes that can be applied to handler functions
#[derive(Debug)]
pub enum AttributeType {
    Message(MessageAttribute),
    Event(EventAttribute),
}

/// Parsed `#[message(pattern = "...")]` attribute
#[derive(Debug, FromAttributes)]
#[darling(attributes(message))]
pub struct MessageAttribute {
    pub pattern: String,
}

/// Parsed `#[event(pattern = "...")]` attribute
#[derive(Debug, FromAttributes)]
#[darling(attributes(event))]
pub struct EventAttribute {
    pub pattern: String,
}

impl AttributeType {
    /// Parse an attribute into an `AttributeType`
    ///
    /// Returns an error if:
    /// - The attribute is not recognized
    /// - The pattern is empty
    /// - The attribute cannot be parsed
    pub fn from_attribute(attr: &Attribute) -> Result<Self, syn::Error> {
        if attr.path().is_ident("message") {
            let parsed = MessageAttribute::from_attributes(&[attr.clone()])?;
            if parsed.pattern.is_empty() {
                return Err(syn::Error::new_spanned(
                    attr,
                    AttributeParseError::MessagePatternIsEmpty.to_string(),
                ));
            }
            Ok(AttributeType::Message(parsed))
        } else if attr.path().is_ident("event") {
            let parsed = EventAttribute::from_attributes(&[attr.clone()])?;
            if parsed.pattern.is_empty() {
                return Err(syn::Error::new_spanned(
                    attr,
                    AttributeParseError::EventPatternIsEmpty.to_string(),
                ));
            }
            Ok(AttributeType::Event(parsed))
        } else {
            Err(syn::Error::new_spanned(
                attr,
                AttributeParseError::UnknownAttribute.to_string(),
            ))
        }
    }

    /// Get the pattern string from the attribute
    pub fn pattern(&self) -> &str {
        match self {
            AttributeType::Message(msg) => &msg.pattern,
            AttributeType::Event(evt) => &evt.pattern,
        }
    }

    /// Check if this is a message attribute
    pub fn is_message(&self) -> bool {
        matches!(self, AttributeType::Message(_))
    }

    /// Check if this is an event attribute
    pub fn is_event(&self) -> bool {
        matches!(self, AttributeType::Event(_))
    }
}

