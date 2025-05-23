//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines, reason = "Generated code")]
use ::core::num::NonZeroU32;
use serde::{Serialize, Deserialize};
#[cfg(feature = "builders")]
use derive_builder::Builder;
use super::super::codes;
#[allow(unused_imports, reason = "Generated code; Integer64 is unused in R4B")]
use crate::{Base64Binary, Date, DateTime, Instant, Time, Integer64};
/** Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.

 **[Address](http://hl7.org/fhir/StructureDefinition/Address) v5.0.0**

 An address expressed using postal conventions (as opposed to GPS or other location definition formats)

 An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.

 Note: address is intended to describe postal addresses for administrative purposes, not to describe absolute geographical coordinates.  Postal addresses are often used as proxies for physical locations (also see the [Location](location.html#) resource). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address(pub Box<AddressInner>);
/** Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.

 **[Address](http://hl7.org/fhir/StructureDefinition/Address) v5.0.0**

 An address expressed using postal conventions (as opposed to GPS or other location definition formats)

 An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.

 Note: address is intended to describe postal addresses for administrative purposes, not to describe absolute geographical coordinates.  Postal addresses are often used as proxies for physical locations (also see the [Location](location.html#) resource). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AddressBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AddressInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[AddressUse](http://hl7.org/fhir/ValueSet/address-use); home | work | temp | old | billing - purpose of this address**

 The purpose of this address.

 Applications can assume that an address is current unless it explicitly says that it is temporary or old. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::AddressUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **[AddressType](http://hl7.org/fhir/ValueSet/address-type); postal | physical | both**

 Distinguishes between physical addresses (those you can visit) and mailing addresses (e.g. PO Boxes and care-of addresses). Most addresses are both.

 The definition of Address states that "address is intended to describe postal addresses, not physical locations". However, many applications track whether an address has a dual purpose of being a location that can be visited as well as being a valid delivery destination, and Postal addresses are often used as proxies for physical locations (also see the [Location](location.html#) resource). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<codes::AddressType>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Text representation of the address**

 Specifies the entire address as it should be displayed e.g. on a postal label. This may be provided instead of or as well as the specific parts.

 Can provide both a text representation and parts. Applications updating an address SHALL ensure that  when both text and parts are present,  no content is included in the text that isn't found in a part. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
    /** **Street name, number, direction & P.O. Box etc.**

 This component contains the house number, apartment number, street name, street direction,  P.O. Box number, delivery hints, and similar address information.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "line")]
    pub line: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_line")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub line_ext: Vec<Option<FieldExtension>>,
    /** **Name of city, town etc.**

 The name of the city, town, suburb, village or other community or delivery center.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "city")]
    pub city: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_city")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub city_ext: Option<FieldExtension>,
    /** **District name (aka county)**

 The name of the administrative area (county).

 District is sometimes known as county, but in some regions 'county' is used in place of city (municipality), so county name should be conveyed in city instead. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "district")]
    pub district: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_district")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub district_ext: Option<FieldExtension>,
    /** **Sub-unit of country (abbreviations ok)**

 Sub-unit of a country with limited sovereignty in a federally organized country. A code may be used if codes are in common use (e.g. US 2 letter state codes).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "state")]
    pub state: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_state")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub state_ext: Option<FieldExtension>,
    /** **Postal code for area**

 A postal code designating a region defined by the postal service.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_postalCode")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub postal_code_ext: Option<FieldExtension>,
    /** **Country (e.g. may be ISO 3166 2 or 3 letter code)**

 Country - a nation as commonly understood or generally accepted.

 ISO 3166 2- or 3- letter codes MAY be used in place of a human readable country name. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "country")]
    pub country: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_country")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub country_ext: Option<FieldExtension>,
    /** **Time period when address was/is in use**

 Time period when address was/is in use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AddressBuilder {
    #[doc = concat!("Finalize building ", "Address", ".")]
    pub fn build(self) -> Result<Address, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AddressInner> for Address {
    fn from(inner: AddressInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Address {
    type Target = AddressInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Address {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Address {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}
/** Age Type: A duration of time during which an organism (or a process) has existed.

 **[Age](http://hl7.org/fhir/StructureDefinition/Age) v5.0.0**

 A duration of time during which an organism (or a process) has existed

 A duration of time during which an organism (or a process) has existed.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Age(pub Box<AgeInner>);
/** Age Type: A duration of time during which an organism (or a process) has existed.

 **[Age](http://hl7.org/fhir/StructureDefinition/Age) v5.0.0**

 A duration of time during which an organism (or a process) has existed

 A duration of time during which an organism (or a process) has existed.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AgeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AgeInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl AgeBuilder {
    #[doc = concat!("Finalize building ", "Age", ".")]
    pub fn build(self) -> Result<Age, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AgeInner> for Age {
    fn from(inner: AgeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Age {
    type Target = AgeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Age {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Age {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AgeBuilder {
        AgeBuilder::default()
    }
}
/** Annotation Type: A  text note which also  contains information about who made the statement and when.

 **[Annotation](http://hl7.org/fhir/StructureDefinition/Annotation) v5.0.0**

 Text node with attribution

 A  text note which also  contains information about who made the statement and when.

 For systems that do not have structured annotations, they can simply communicate a single annotation with no author or time.  This element may need to be included in narrative because of the potential for modifying information.  *Annotations SHOULD NOT* be used to communicate "modifying" information that could be computable. (This is a SHOULD because enforcing user behavior is nearly impossible). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Annotation(pub Box<AnnotationInner>);
/** Annotation Type: A  text note which also  contains information about who made the statement and when.

 **[Annotation](http://hl7.org/fhir/StructureDefinition/Annotation) v5.0.0**

 Text node with attribution

 A  text note which also  contains information about who made the statement and when.

 For systems that do not have structured annotations, they can simply communicate a single annotation with no author or time.  This element may need to be included in narrative because of the potential for modifying information.  *Annotations SHOULD NOT* be used to communicate "modifying" information that could be computable. (This is a SHOULD because enforcing user behavior is nearly impossible). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AnnotationBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AnnotationInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Individual responsible for the annotation**

 The individual responsible for making the annotation.

 Organization is used when there's no need for specific attribution as to who made the comment. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub author: Option<AnnotationAuthor>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub author_ext: Option<AnnotationAuthorExtension>,
    /** **When the annotation was made**

 Indicates when this particular annotation was made.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "time")]
    pub time: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_time")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub time_ext: Option<FieldExtension>,
    /** **The annotation  - text content (as markdown)**

 The text of the annotation in markdown format.

 */
    #[serde(rename = "text")]
    pub text: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AnnotationBuilder {
    #[doc = concat!("Finalize building ", "Annotation", ".")]
    pub fn build(self) -> Result<Annotation, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AnnotationInner> for Annotation {
    fn from(inner: AnnotationInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Annotation {
    type Target = AnnotationInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Annotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Annotation {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AnnotationBuilder {
        AnnotationBuilder::default()
    }
}
/// Choice of types for the author[x] field in Annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnnotationAuthor {
    /// Variant accepting the Reference type.
    #[serde(rename = "authorReference")]
    Reference(Reference),
    /// Variant accepting the String type.
    #[serde(rename = "authorString")]
    String(String),
}
/// Extension value for AnnotationAuthor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnnotationAuthorExtension {
    /// Variant accepting the Reference extension.
    #[serde(rename = "_authorReference")]
    Reference(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_authorString")]
    String(FieldExtension),
}
/** Attachment Type: For referring to data content defined in other formats.

 **[Attachment](http://hl7.org/fhir/StructureDefinition/Attachment) v5.0.0**

 Content in a format defined elsewhere

 For referring to data content defined in other formats.

 When providing a summary view (for example with Observation.value[x]) Attachment should be represented with a brief display text such as "Signed Procedure Consent". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Attachment(pub Box<AttachmentInner>);
/** Attachment Type: For referring to data content defined in other formats.

 **[Attachment](http://hl7.org/fhir/StructureDefinition/Attachment) v5.0.0**

 Content in a format defined elsewhere

 For referring to data content defined in other formats.

 When providing a summary view (for example with Observation.value[x]) Attachment should be represented with a brief display text such as "Signed Procedure Consent". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AttachmentBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AttachmentInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[MimeType](http://hl7.org/fhir/ValueSet/mimetypes); Mime type of the content, with charset etc.**

 Identifies the type of the data in the attachment and allows a method to be chosen to interpret or render the data. Includes mime type parameters such as charset where appropriate.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_contentType")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub content_type_ext: Option<FieldExtension>,
    /** **[Language](http://hl7.org/fhir/ValueSet/all-languages); Human language of the content (BCP-47)**

 The human language of the content. The value can be any valid value according to BCP 47.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_language")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub language_ext: Option<FieldExtension>,
    /** **Data inline, base64ed**

 The actual data of the attachment - a sequence of bytes, base64 encoded.

 The base64-encoded data SHALL be expressed in the same character set as the base resource XML or JSON. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "data")]
    pub data: Option<Base64Binary>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_data")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub data_ext: Option<FieldExtension>,
    /** **Uri where the data can be found**

 A location where the data can be accessed.

 If both data and url are provided, the url SHALL point to the same content as the data contains. Urls may be relative references or may reference transient locations such as a wrapping envelope using cid: though this has ramifications for using signatures. Relative URLs are interpreted relative to the service url, like a resource reference, rather than relative to the resource itself. If a URL is provided, it SHALL resolve to actual data. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_url")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub url_ext: Option<FieldExtension>,
    /** **Number of bytes of content (if url provided)**

 The number of bytes of data that make up this attachment (before base64 encoding, if that is done).

 The number of bytes is redundant if the data is provided as a base64binary, but is useful if the data is provided as a url reference. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "size")]
    pub size: Option<Integer64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_size")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub size_ext: Option<FieldExtension>,
    /** **Hash of the data (sha-1, base64ed)**

 The calculated hash of the data using SHA-1. Represented using base64.

 The hash is calculated on the data prior to base64 encoding, if the data is based64 encoded. The hash is not intended to support digital signatures. Where protection against malicious threats a digital signature should be considered, see [Provenance.signature](provenance-definitions.html#Provenance.signature) for mechanism to protect a resource with a digital signature. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "hash")]
    pub hash: Option<Base64Binary>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_hash")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub hash_ext: Option<FieldExtension>,
    /** **Label to display in place of the data**

 A label or set of text to display in place of the data.

 May sometimes be derived from the source filename. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_title")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub title_ext: Option<FieldExtension>,
    /** **Date attachment was first created**

 The date that the attachment was first created.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "creation")]
    pub creation: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_creation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub creation_ext: Option<FieldExtension>,
    /** **Height of the image in pixels (photo/video)**

 Height of the image in pixels (photo/video).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "height")]
    pub height: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_height")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub height_ext: Option<FieldExtension>,
    /** **Width of the image in pixels (photo/video)**

 Width of the image in pixels (photo/video).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "width")]
    pub width: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_width")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub width_ext: Option<FieldExtension>,
    /** **Number of frames if > 1 (photo)**

 The number of frames in a photo. This is used with a multi-page fax, or an imaging acquisition context that takes multiple slices in a single image, or an animated gif. If there is more than one frame, this SHALL have a value in order to alert interface software that a multi-frame capable rendering widget is required.

 if the number of frames is not supplied, the value may be unknown. Applications should not assume that there is only one frame unless it is explicitly stated. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "frames")]
    pub frames: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_frames")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub frames_ext: Option<FieldExtension>,
    /** **Length in seconds (audio / video)**

 The duration of the recording in seconds - for audio and video.

 The duration might differ from occurrencePeriod if recording was paused. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_duration")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_ext: Option<FieldExtension>,
    /** **Number of printed pages**

 The number of pages when printed.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "pages")]
    pub pages: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_pages")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub pages_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AttachmentBuilder {
    #[doc = concat!("Finalize building ", "Attachment", ".")]
    pub fn build(self) -> Result<Attachment, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AttachmentInner> for Attachment {
    fn from(inner: AttachmentInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Attachment {
    type Target = AttachmentInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Attachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Attachment {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AttachmentBuilder {
        AttachmentBuilder::default()
    }
}
/** Availability Type: Availability data for an {item}.

 **[Availability](http://hl7.org/fhir/StructureDefinition/Availability) v5.0.0**

 Availability data for an {item}

 Availability data for an {item}.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Availability(pub Box<AvailabilityInner>);
/** Availability Type: Availability data for an {item}.

 **[Availability](http://hl7.org/fhir/StructureDefinition/Availability) v5.0.0**

 Availability data for an {item}

 Availability data for an {item}.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AvailabilityBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AvailabilityInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Times the {item} is available**

 Times the {item} is available.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "availableTime")]
    pub available_time: Vec<Option<AvailabilityAvailableTime>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_availableTime")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub available_time_ext: Vec<Option<FieldExtension>>,
    /** **Not available during this time due to provided reason**

 Not available during this time due to provided reason.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "notAvailableTime")]
    pub not_available_time: Vec<Option<AvailabilityNotAvailableTime>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_notAvailableTime")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub not_available_time_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl AvailabilityBuilder {
    #[doc = concat!("Finalize building ", "Availability", ".")]
    pub fn build(self) -> Result<Availability, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AvailabilityInner> for Availability {
    fn from(inner: AvailabilityInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Availability {
    type Target = AvailabilityInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Availability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Availability {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AvailabilityBuilder {
        AvailabilityBuilder::default()
    }
}
/// Sub-fields of the availableTime field in Availability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AvailabilityAvailableTimeBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct AvailabilityAvailableTime {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[DaysOfWeek](http://hl7.org/fhir/ValueSet/days-of-week); mon | tue | wed | thu | fri | sat | sun**

 mon | tue | wed | thu | fri | sat | sun.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Vec<Option<codes::DaysOfWeek>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_daysOfWeek")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub days_of_week_ext: Vec<Option<FieldExtension>>,
    /** **Always available? i.e. 24 hour service**

 Always available? i.e. 24 hour service.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "allDay")]
    pub all_day: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_allDay")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub all_day_ext: Option<FieldExtension>,
    /** **Opening time of day (ignored if allDay = true)**

 Opening time of day (ignored if allDay = true).

 The timezone is expected to be specified or implied by the context this datatype is used. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "availableStartTime")]
    pub available_start_time: Option<Time>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_availableStartTime")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub available_start_time_ext: Option<FieldExtension>,
    /** **Closing time of day (ignored if allDay = true)**

 Closing time of day (ignored if allDay = true).

 The timezone is expected to be specified or implied by the context this datatype is used. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "availableEndTime")]
    pub available_end_time: Option<Time>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_availableEndTime")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub available_end_time_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AvailabilityAvailableTime {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> AvailabilityAvailableTimeBuilder {
        AvailabilityAvailableTimeBuilder::default()
    }
}
/// Sub-fields of the notAvailableTime field in Availability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AvailabilityNotAvailableTimeBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct AvailabilityNotAvailableTime {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Reason presented to the user explaining why time not available**

 Reason presented to the user explaining why time not available.

 The reason will generally be provided to give the textual reason for displaying when the {item} is not available, e.g. 'Closed public holidays' or 'Independence Day'. In cases such as this, the `during` might not be included and local knowledge would be required in such cases (as don't desire to keep updating when the holiday occurs each year).

e.g.2: 'Closed for maintenance over the summer' for this example you would want to include the `during` period, unless this was a university hospital and the "summer" period was well known, but would recommend its inclusion anyway. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_description")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub description_ext: Option<FieldExtension>,
    /** **Service not available during this period**

 Service not available during this period.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "during")]
    pub during: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_during")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub during_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AvailabilityNotAvailableTime {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> AvailabilityNotAvailableTimeBuilder {
        AvailabilityNotAvailableTimeBuilder::default()
    }
}
/** CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.

 **[CodeableConcept](http://hl7.org/fhir/StructureDefinition/CodeableConcept) v5.0.0**

 Concept - reference to a terminology or just  text

 A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.

 Not all terminology uses fit this general pattern. In some cases, models should not use CodeableConcept and use Coding directly and provide their own structure for managing text, codings, translations and the relationship between elements and pre- and post-coordination. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CodeableConcept(pub Box<CodeableConceptInner>);
/** CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.

 **[CodeableConcept](http://hl7.org/fhir/StructureDefinition/CodeableConcept) v5.0.0**

 Concept - reference to a terminology or just  text

 A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.

 Not all terminology uses fit this general pattern. In some cases, models should not use CodeableConcept and use Coding directly and provide their own structure for managing text, codings, translations and the relationship between elements and pre- and post-coordination. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CodeableConceptBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CodeableConceptInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Code defined by a terminology system**

 A reference to a code defined by a terminology system.

 Codes may be defined very casually in enumerations, or code lists, up to very formal definitions such as SNOMED CT - see the HL7 v3 Core Principles for more information.  Ordering of codings is undefined and SHALL NOT be used to infer meaning. Generally, at most only one of the coding values will be labeled as UserSelected = true. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "coding")]
    pub coding: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_coding")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub coding_ext: Vec<Option<FieldExtension>>,
    /** **Plain text representation of the concept**

 A human language representation of the concept as seen/selected/uttered by the user who entered the data and/or which represents the intended meaning of the user.

 Very often the text is the same as a displayName of one of the codings. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl CodeableConceptBuilder {
    #[doc = concat!("Finalize building ", "CodeableConcept", ".")]
    pub fn build(self) -> Result<CodeableConcept, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CodeableConceptInner> for CodeableConcept {
    fn from(inner: CodeableConceptInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for CodeableConcept {
    type Target = CodeableConceptInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for CodeableConcept {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl CodeableConcept {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CodeableConceptBuilder {
        CodeableConceptBuilder::default()
    }
}
/** CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).

 **[CodeableReference](http://hl7.org/fhir/StructureDefinition/CodeableReference) v5.0.0**

 Reference to a resource or a concept

 A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CodeableReference(pub Box<CodeableReferenceInner>);
/** CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).

 **[CodeableReference](http://hl7.org/fhir/StructureDefinition/CodeableReference) v5.0.0**

 Reference to a resource or a concept

 A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CodeableReferenceBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CodeableReferenceInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Reference to a concept (by class)**

 A reference to a concept - e.g. the information is identified by its general class to the degree of precision found in the terminology.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "concept")]
    pub concept: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_concept")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub concept_ext: Option<FieldExtension>,
    /** **Reference to a resource (by instance)**

 A reference to a resource the provides exact details about the information being referenced.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "reference")]
    pub reference: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_reference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub reference_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl CodeableReferenceBuilder {
    #[doc = concat!("Finalize building ", "CodeableReference", ".")]
    pub fn build(self) -> Result<CodeableReference, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CodeableReferenceInner> for CodeableReference {
    fn from(inner: CodeableReferenceInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for CodeableReference {
    type Target = CodeableReferenceInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for CodeableReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl CodeableReference {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CodeableReferenceBuilder {
        CodeableReferenceBuilder::default()
    }
}
/** Coding Type: A reference to a code defined by a terminology system.

 **[Coding](http://hl7.org/fhir/StructureDefinition/Coding) v5.0.0**

 A reference to a code defined by a terminology system

 A reference to a code defined by a terminology system.

 Codes may be defined very casually in enumerations or code lists, up to very formal definitions such as SNOMED CT - see the HL7 v3 Core Principles for more information. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Coding(pub Box<CodingInner>);
/** Coding Type: A reference to a code defined by a terminology system.

 **[Coding](http://hl7.org/fhir/StructureDefinition/Coding) v5.0.0**

 A reference to a code defined by a terminology system

 A reference to a code defined by a terminology system.

 Codes may be defined very casually in enumerations or code lists, up to very formal definitions such as SNOMED CT - see the HL7 v3 Core Principles for more information. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CodingBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CodingInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Identity of the terminology system**

 The identification of the code system that defines the meaning of the symbol in the code.

 The URI may be an OID (urn:oid:...) or a UUID (urn:uuid:...).  OIDs and UUIDs SHALL be references to the HL7 OID registry. Otherwise, the URI should come from HL7's list of FHIR defined special URIs or it should be an absolute reference to some definition that establishes the system clearly and unambiguously. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Version of the system - if relevant**

 The version of the code system which was used when choosing this code. Note that a well-maintained code system does not need the version reported, because the meaning of codes is consistent across versions. However this cannot consistently be assured, and when the meaning is not guaranteed to be consistent, the version SHOULD be exchanged.

 Where the terminology does not clearly define what string should be used to identify code system versions, the recommendation is to use the date (expressed in FHIR date format) on which that version was officially published as the version date. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "version")]
    pub version: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_version")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub version_ext: Option<FieldExtension>,
    /** **Symbol in syntax defined by the system**

 A symbol in syntax defined by the system. The symbol may be a predefined code or an expression in a syntax defined by the coding system (e.g. post-coordination).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Representation defined by the system**

 A representation of the meaning of the code in the system, following the rules of the system.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "display")]
    pub display: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_display")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub display_ext: Option<FieldExtension>,
    /** **If this coding was chosen directly by the user**

 Indicates that this coding was chosen by a user directly - e.g. off a pick list of available items (codes or displays).

 Amongst a set of alternatives, a directly chosen code is the most appropriate starting point for new translations. There is some ambiguity about what exactly 'directly chosen' implies, and trading partner agreement may be needed to clarify the use of this element and its consequences more completely. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "userSelected")]
    pub user_selected: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_userSelected")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub user_selected_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl CodingBuilder {
    #[doc = concat!("Finalize building ", "Coding", ".")]
    pub fn build(self) -> Result<Coding, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CodingInner> for Coding {
    fn from(inner: CodingInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Coding {
    type Target = CodingInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Coding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Coding {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CodingBuilder {
        CodingBuilder::default()
    }
}
/** ContactDetail Type: Specifies contact information for a person or organization.

 **[ContactDetail](http://hl7.org/fhir/StructureDefinition/ContactDetail) v5.0.0**

 Contact information

 Specifies contact information for a person or organization.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContactDetail(pub Box<ContactDetailInner>);
/** ContactDetail Type: Specifies contact information for a person or organization.

 **[ContactDetail](http://hl7.org/fhir/StructureDefinition/ContactDetail) v5.0.0**

 Contact information

 Specifies contact information for a person or organization.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ContactDetailBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ContactDetailInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Name of an individual to contact**

 The name of an individual to contact.

 If there is no named individual, the telecom information is for the organization as a whole. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **Contact details for individual or organization**

 The contact details for the individual (if a name was provided) or the organization.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "telecom")]
    pub telecom: Vec<Option<ContactPoint>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_telecom")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub telecom_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ContactDetailBuilder {
    #[doc = concat!("Finalize building ", "ContactDetail", ".")]
    pub fn build(self) -> Result<ContactDetail, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ContactDetailInner> for ContactDetail {
    fn from(inner: ContactDetailInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ContactDetail {
    type Target = ContactDetailInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ContactDetail {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ContactDetail {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ContactDetailBuilder {
        ContactDetailBuilder::default()
    }
}
/** ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.

 **[ContactPoint](http://hl7.org/fhir/StructureDefinition/ContactPoint) v5.0.0**

 Details of a Technology mediated contact point (phone, fax, email, etc.)

 Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContactPoint(pub Box<ContactPointInner>);
/** ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.

 **[ContactPoint](http://hl7.org/fhir/StructureDefinition/ContactPoint) v5.0.0**

 Details of a Technology mediated contact point (phone, fax, email, etc.)

 Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ContactPointBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ContactPointInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[ContactPointSystem](http://hl7.org/fhir/ValueSet/contact-point-system); phone | fax | email | pager | url | sms | other**

 Telecommunications form for contact point - what communications system is required to make use of the contact.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<codes::ContactPointSystem>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **The actual contact point details**

 The actual contact point details, in a form that is meaningful to the designated communication system (i.e. phone number or email address).

 Additional text data such as phone extension numbers, or notes about use of the contact are sometimes included in the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[ContactPointUse](http://hl7.org/fhir/ValueSet/contact-point-use); home | work | temp | old | mobile - purpose of this contact point**

 Identifies the purpose for the contact point.

 Applications can assume that a contact is current unless it explicitly says that it is temporary or old. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::ContactPointUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **Specify preferred order of use (1 = highest)**

 Specifies a preferred order in which to use a set of contacts. ContactPoints with lower rank values are more preferred than those with higher rank values.

 Note that rank does not necessarily follow the order in which the contacts are represented in the instance.

Ranks need not be unique.  E.g. it's possible to have multiple contacts with rank=1.  If the ranks have different systems or uses, this would be interpreted to mean "X is my most preferred phone number, Y is my most preferred email address" or "X is my preferred home email and Y is my preferred work email".  If the system and use for equally-ranked contacts are the same, then the level of preference is equivalent for both repetitions.

Ranks need not be sequential and not all repetitions must have a rank.  For example, it's possible to have 4 contacts with ranks of 2, 5 and two with no rank specified.  That would be interpreted to mean the first is preferred over the second and no preference stated for the remaining contacts. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "rank")]
    pub rank: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_rank")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub rank_ext: Option<FieldExtension>,
    /** **Time period when the contact point was/is in use**

 Time period when the contact point was/is in use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ContactPointBuilder {
    #[doc = concat!("Finalize building ", "ContactPoint", ".")]
    pub fn build(self) -> Result<ContactPoint, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ContactPointInner> for ContactPoint {
    fn from(inner: ContactPointInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ContactPoint {
    type Target = ContactPointInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ContactPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ContactPoint {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ContactPointBuilder {
        ContactPointBuilder::default()
    }
}
/** Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.

 **[Contributor](http://hl7.org/fhir/StructureDefinition/Contributor) v5.0.0**

 Contributor information

 A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Contributor(pub Box<ContributorInner>);
/** Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.

 **[Contributor](http://hl7.org/fhir/StructureDefinition/Contributor) v5.0.0**

 Contributor information

 A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ContributorBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ContributorInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[ContributorType](http://hl7.org/fhir/ValueSet/contributor-type); author | editor | reviewer | endorser**

 The type of contributor.

 */
    #[serde(rename = "type")]
    pub r#type: codes::ContributorType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Who contributed the content**

 The name of the individual or organization responsible for the contribution.

 */
    #[serde(rename = "name")]
    pub name: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **Contact details of the contributor**

 Contact details to assist a user in finding and communicating with the contributor.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contact")]
    pub contact: Vec<Option<ContactDetail>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_contact")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub contact_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ContributorBuilder {
    #[doc = concat!("Finalize building ", "Contributor", ".")]
    pub fn build(self) -> Result<Contributor, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ContributorInner> for Contributor {
    fn from(inner: ContributorInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Contributor {
    type Target = ContributorInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Contributor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Contributor {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ContributorBuilder {
        ContributorBuilder::default()
    }
}
/** Count Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 **[Count](http://hl7.org/fhir/StructureDefinition/Count) v5.0.0**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Count(pub Box<CountInner>);
/** Count Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 **[Count](http://hl7.org/fhir/StructureDefinition/Count) v5.0.0**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CountBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CountInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl CountBuilder {
    #[doc = concat!("Finalize building ", "Count", ".")]
    pub fn build(self) -> Result<Count, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CountInner> for Count {
    fn from(inner: CountInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Count {
    type Target = CountInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Count {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Count {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CountBuilder {
        CountBuilder::default()
    }
}
/** DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.

 **[DataRequirement](http://hl7.org/fhir/StructureDefinition/DataRequirement) v5.0.0**

 Describes a required data item

 Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataRequirement(pub Box<DataRequirementInner>);
/** DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.

 **[DataRequirement](http://hl7.org/fhir/StructureDefinition/DataRequirement) v5.0.0**

 Describes a required data item

 Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DataRequirementInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[FHIRTypes](http://hl7.org/fhir/ValueSet/fhir-types); The type of the required data**

 The type of the required data, specified as the type name of a resource. For profiles, this value is set to the type of the base resource of the profile.

 */
    #[serde(rename = "type")]
    pub r#type: codes::FHIRTypes,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **The profile of the required data**

 The profile of the required data, specified as the uri of the profile definition.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub profile_ext: Vec<Option<FieldExtension>>,
    /** **E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device**

 The intended subjects of the data requirement. If this element is not provided, a Patient subject is assumed.

 The subject of a data requirement is critical, as the data being specified is determined with respect to a particular subject. This corresponds roughly to the notion of a Compartment in that it limits what data is available based on its relationship to the subject. In CQL, this corresponds to the context declaration. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub subject: Option<DataRequirementSubject>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub subject_ext: Option<DataRequirementSubjectExtension>,
    /** **Indicates specific structure elements that are referenced by the knowledge module**

 Indicates that specific elements of the type are referenced by the knowledge module and must be supported by the consumer in order to obtain an effective evaluation. This does not mean that a value is required for this element, only that the consuming system must understand the element and be able to provide values for it if they are available.

The value of mustSupport SHALL be a FHIRPath resolvable on the type of the DataRequirement. The path SHALL consist only of identifiers, constant indexers, and .resolve() (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details).

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mustSupport")]
    pub must_support: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_mustSupport")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub must_support_ext: Vec<Option<FieldExtension>>,
    /** **What codes are expected**

 Code filters specify additional constraints on the data, specifying the value set of interest for a particular element of the data. Each code filter defines an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "codeFilter")]
    pub code_filter: Vec<Option<DataRequirementCodeFilter>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_codeFilter")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub code_filter_ext: Vec<Option<FieldExtension>>,
    /** **What dates/date ranges are expected**

 Date filters specify additional constraints on the data in terms of the applicable date range for specific elements. Each date filter specifies an additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "dateFilter")]
    pub date_filter: Vec<Option<DataRequirementDateFilter>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_dateFilter")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub date_filter_ext: Vec<Option<FieldExtension>>,
    /** **What values are expected**

 Value filters specify additional constraints on the data for elements other than code-valued or date-valued. Each value filter specifies an additional constraint on the data (i.e. valueFilters are AND'ed, not OR'ed).

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueFilter")]
    pub value_filter: Vec<Option<DataRequirementValueFilter>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_valueFilter")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub value_filter_ext: Vec<Option<FieldExtension>>,
    /** **Number of results**

 Specifies a maximum number of results that are required (uses the _count search parameter).

 This element can be used in combination with the sort element to specify quota requirements such as "the most recent 5" or "the highest 5". */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "limit")]
    pub limit: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_limit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub limit_ext: Option<FieldExtension>,
    /** **Order of the results**

 Specifies the order of the results to be returned.

 This element can be used in combination with the sort element to specify quota requirements such as "the most recent 5" or "the highest 5". When multiple sorts are specified, they are applied in the order they appear in the resource. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sort")]
    pub sort: Vec<Option<DataRequirementSort>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_sort")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub sort_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl DataRequirementBuilder {
    #[doc = concat!("Finalize building ", "DataRequirement", ".")]
    pub fn build(self) -> Result<DataRequirement, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DataRequirementInner> for DataRequirement {
    fn from(inner: DataRequirementInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for DataRequirement {
    type Target = DataRequirementInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for DataRequirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DataRequirement {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DataRequirementBuilder {
        DataRequirementBuilder::default()
    }
}
/// Choice of types for the subject[x] field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementSubject {
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "subjectCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Reference type.
    #[serde(rename = "subjectReference")]
    Reference(Reference),
}
/// Extension value for DataRequirementSubject.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementSubjectExtension {
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_subjectCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_subjectReference")]
    Reference(FieldExtension),
}
/// Sub-fields of the codeFilter field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementCodeFilterBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DataRequirementCodeFilter {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **A code-valued attribute to filter on**

 The code-valued attribute of the filter. The specified path SHALL be a FHIRPath resolvable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of type code, Coding, or CodeableConcept.

 The path attribute contains a [Simple FHIRPath Subset](fhirpath.html#simple) that allows path traversal, but not calculation. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **A coded (token) parameter to search on**

 A token parameter that refers to a search parameter defined on the specified type of the DataRequirement, and which searches on elements of type code, Coding, or CodeableConcept.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "searchParam")]
    pub search_param: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_searchParam")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub search_param_ext: Option<FieldExtension>,
    /** **ValueSet for the filter**

 The valueset for the code filter. The valueSet and code elements are additive. If valueSet is specified, the filter will return only those data items for which the value of the code-valued element specified in the path is a member of the specified valueset.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_valueSet")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_set_ext: Option<FieldExtension>,
    /** **What code is expected**

 The codes for the code filter. If values are given, the filter will return only those data items for which the code-valued attribute specified by the path has a value that is one of the specified codes. If codes are specified in addition to a value set, the filter returns items matching a code in the value set or one of the specified codes.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub code_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl DataRequirementCodeFilter {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DataRequirementCodeFilterBuilder {
        DataRequirementCodeFilterBuilder::default()
    }
}
/// Sub-fields of the dateFilter field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementDateFilterBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DataRequirementDateFilter {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **A date-valued attribute to filter on**

 The date-valued attribute of the filter. The specified path SHALL be a FHIRPath resolvable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of type date, dateTime, Period, Schedule, or Timing.

 The path attribute contains a [Simple FHIR Subset](fhirpath.html#simple) that allows path traversal, but not calculation. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **A date valued parameter to search on**

 A date parameter that refers to a search parameter defined on the specified type of the DataRequirement, and which searches on elements of type date, dateTime, Period, Schedule, or Timing.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "searchParam")]
    pub search_param: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_searchParam")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub search_param_ext: Option<FieldExtension>,
    /** **The value of the filter, as a Period, DateTime, or Duration value**

 The value of the filter. If period is specified, the filter will return only those data items that fall within the bounds determined by the Period, inclusive of the period boundaries. If dateTime is specified, the filter will return only those data items that are equal to the specified dateTime. If a Duration is specified, the filter will return only those data items that fall within Duration before now.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value: Option<DataRequirementDateFilterValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<DataRequirementDateFilterValueExtension>,
}
#[cfg(feature = "builders")]
impl DataRequirementDateFilter {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DataRequirementDateFilterBuilder {
        DataRequirementDateFilterBuilder::default()
    }
}
/// Choice of types for the value[x] field in DataRequirementDateFilter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementDateFilterValue {
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
}
/// Extension value for DataRequirementDateFilterValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementDateFilterValueExtension {
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
}
/// Sub-fields of the valueFilter field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementValueFilterBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DataRequirementValueFilter {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **An attribute to filter on**

 The attribute of the filter. The specified path SHALL be a FHIRPath resolvable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of a type that is comparable to the valueFilter.value[x] element for the filter.

 The path attribute contains a [Simple FHIR Subset](fhirpath.html#simple) that allows path traversal, but not calculation. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **A parameter to search on**

 A search parameter defined on the specified type of the DataRequirement, and which searches on elements of a type compatible with the type of the valueFilter.value[x] for the filter.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "searchParam")]
    pub search_param: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_searchParam")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub search_param_ext: Option<FieldExtension>,
    /** **[ValueFilterComparator](http://hl7.org/fhir/ValueSet/value-filter-comparator); eq | gt | lt | ge | le | sa | eb**

 The comparator to be used to determine whether the value is matching.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comparator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comparator_ext: Option<FieldExtension>,
    /** **The value of the filter, as a Period, DateTime, or Duration value**

 The value of the filter.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value: Option<DataRequirementValueFilterValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<DataRequirementValueFilterValueExtension>,
}
#[cfg(feature = "builders")]
impl DataRequirementValueFilter {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DataRequirementValueFilterBuilder {
        DataRequirementValueFilterBuilder::default()
    }
}
/// Choice of types for the value[x] field in DataRequirementValueFilter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementValueFilterValue {
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
}
/// Extension value for DataRequirementValueFilterValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementValueFilterValueExtension {
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
}
/// Sub-fields of the sort field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementSortBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DataRequirementSort {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **The name of the attribute to perform the sort**

 The attribute of the sort. The specified path must be resolvable from the type of the required data. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-elements. Note that the index must be an integer constant.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **[SortDirection](http://hl7.org/fhir/ValueSet/sort-direction); ascending | descending**

 The direction of the sort, ascending or descending.

 */
    #[serde(rename = "direction")]
    pub direction: codes::SortDirection,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_direction")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub direction_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl DataRequirementSort {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DataRequirementSortBuilder {
        DataRequirementSortBuilder::default()
    }
}
/** Distance Type: A length - a value with a unit that is a physical distance.

 **[Distance](http://hl7.org/fhir/StructureDefinition/Distance) v5.0.0**

 A length - a value with a unit that is a physical distance

 A length - a value with a unit that is a physical distance.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Distance(pub Box<DistanceInner>);
/** Distance Type: A length - a value with a unit that is a physical distance.

 **[Distance](http://hl7.org/fhir/StructureDefinition/Distance) v5.0.0**

 A length - a value with a unit that is a physical distance

 A length - a value with a unit that is a physical distance.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DistanceBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DistanceInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl DistanceBuilder {
    #[doc = concat!("Finalize building ", "Distance", ".")]
    pub fn build(self) -> Result<Distance, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DistanceInner> for Distance {
    fn from(inner: DistanceInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Distance {
    type Target = DistanceInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Distance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Distance {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DistanceBuilder {
        DistanceBuilder::default()
    }
}
/** Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.

 **[Dosage](http://hl7.org/fhir/StructureDefinition/Dosage) v5.0.0**

 How the medication is/was taken or should be taken

 Indicates how the medication is/was taken or should be taken by the patient.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Dosage(pub Box<DosageInner>);
/** Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.

 **[Dosage](http://hl7.org/fhir/StructureDefinition/Dosage) v5.0.0**

 How the medication is/was taken or should be taken

 Indicates how the medication is/was taken or should be taken by the patient.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DosageBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DosageInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Extensions that cannot be ignored even if unrecognized**

 May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.

Modifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself).

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Vec<Extension>,
    /** **The order of the dosage instructions**

 Indicates the order in which the dosage instructions should be applied or interpreted.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sequence")]
    pub sequence: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sequence")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub sequence_ext: Option<FieldExtension>,
    /** **Free text dosage instructions e.g. SIG**

 Free text dosage instructions e.g. SIG.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
    /** **[AdditionalInstruction](http://hl7.org/fhir/ValueSet/additional-instruction-codes); Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"**

 Supplemental instructions to the patient on how to take the medication  (e.g. "with meals" or"take half to one hour before food") or warnings for the patient about the medication (e.g. "may cause drowsiness" or "avoid exposure of skin to direct sunlight or sunlamps").

 Information about administration or preparation of the medication (e.g. "infuse as rapidly as possibly via intraperitoneal port" or take "immediately following drug x") should be populated in dosage.text. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "additionalInstruction")]
    pub additional_instruction: Vec<Option<CodeableConcept>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_additionalInstruction")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub additional_instruction_ext: Vec<Option<FieldExtension>>,
    /** **Patient or consumer oriented instructions**

 Instructions in terms that are understood by the patient or consumer.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_patientInstruction")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub patient_instruction_ext: Option<FieldExtension>,
    /** **When medication should be administered**

 When medication should be administered.

 This attribute might not always be populated while the Dosage.text is expected to be populated.  If both are populated, then the Dosage.text should reflect the content of the Dosage.timing. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "timing")]
    pub timing: Option<Timing>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_timing")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub timing_ext: Option<FieldExtension>,
    /** **Take "as needed"**

 Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option).

 Can express "as needed" without a reason by setting the Boolean = True.  In this case the CodeableConcept is not populated. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_asNeeded")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub as_needed_ext: Option<FieldExtension>,
    /** **[MedicationAsNeededReason](http://hl7.org/fhir/ValueSet/medication-as-needed-reason); Take "as needed" (for x)**

 Indicates whether the Medication is only taken based on a precondition for taking the Medication (CodeableConcept).

 Can express "as needed" with a reason by including the CodeableConcept.  In this case the Boolean is assumed to be False, then the dose is given according to the schedule and is not "prn" or "as needed". */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "asNeededFor")]
    pub as_needed_for: Vec<Option<CodeableConcept>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_asNeededFor")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub as_needed_for_ext: Vec<Option<FieldExtension>>,
    /** **[MedicationAdministrationSite](http://hl7.org/fhir/ValueSet/approach-site-codes); Body site to administer to**

 Body site to administer to.

 If the use case requires attributes from the BodySite resource (e.g. to identify and track separately) then use the standard extension [http://hl7.org/fhir/StructureDefinition/bodySite](http://hl7.org/fhir/extensions/StructureDefinition-bodySite.html).  May be a summary code, or a reference to a very precise definition of the location, or both. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "site")]
    pub site: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_site")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub site_ext: Option<FieldExtension>,
    /** **[RouteOfAdministration](http://hl7.org/fhir/ValueSet/route-codes); How drug should enter body**

 How drug should enter body.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "route")]
    pub route: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_route")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub route_ext: Option<FieldExtension>,
    /** **[MedicationAdministrationMethod](http://hl7.org/fhir/ValueSet/administration-method-codes); Technique for administering medication**

 Technique for administering medication.

 Terminologies used often pre-coordinate this term with the route and or form of administration. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "method")]
    pub method: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_method")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub method_ext: Option<FieldExtension>,
    /** **Amount of medication administered, to be administered or typical amount to be administered**

 Depending on the resource,this is the amount of medication administered, to  be administered or typical amount to be administered.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "doseAndRate")]
    pub dose_and_rate: Vec<Option<DosageDoseAndRate>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_doseAndRate")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub dose_and_rate_ext: Vec<Option<FieldExtension>>,
    /** **Upper limit on medication per unit of time**

 Upper limit on medication per unit of time.

 This is intended for use as an adjunct to the dosage when there is an upper cap.  For example "2 tablets every 4 hours to a maximum of 8/day". */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxDosePerPeriod")]
    pub max_dose_per_period: Vec<Option<Ratio>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_maxDosePerPeriod")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub max_dose_per_period_ext: Vec<Option<FieldExtension>>,
    /** **Upper limit on medication per administration**

 Upper limit on medication per administration.

 This is intended for use as an adjunct to the dosage when there is an upper cap.  For example, a body surface area related dose with a maximum amount, such as 1.5 mg/m2 (maximum 2 mg) IV over 5 – 10 minutes would have doseQuantity of 1.5 mg/m2 and maxDosePerAdministration of 2 mg. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxDosePerAdministration")]
    pub max_dose_per_administration: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxDosePerAdministration")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_dose_per_administration_ext: Option<FieldExtension>,
    /** **Upper limit on medication per lifetime of the patient**

 Upper limit on medication per lifetime of the patient.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxDosePerLifetime")]
    pub max_dose_per_lifetime: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxDosePerLifetime")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_dose_per_lifetime_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl DosageBuilder {
    #[doc = concat!("Finalize building ", "Dosage", ".")]
    pub fn build(self) -> Result<Dosage, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DosageInner> for Dosage {
    fn from(inner: DosageInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Dosage {
    type Target = DosageInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Dosage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Dosage {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DosageBuilder {
        DosageBuilder::default()
    }
}
/// Sub-fields of the doseAndRate field in Dosage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DosageDoseAndRateBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DosageDoseAndRate {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[DoseAndRateType](http://terminology.hl7.org/ValueSet/dose-rate-type); The kind of dose or rate specified**

 The kind of dose or rate specified, for example, ordered or calculated.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Amount of medication per dose**

 Amount of medication per dose.

 Note that this specifies the quantity of the specified medication, not the quantity for each active ingredient(s). Each ingredient amount can be communicated in the Medication resource. For example, if one wants to communicate that a tablet was 375 mg, where the dose was one tablet, you can use the Medication resource to document that the tablet was comprised of 375 mg of drug XYZ. Alternatively if the dose was 375 mg, then you may only need to use the Medication resource to indicate this was a tablet. If the example were an IV such as dopamine and you wanted to communicate that 400mg of dopamine was mixed in 500 ml of some IV solution, then this would all be communicated in the Medication resource. If the administration is not intended to be instantaneous (rate is present or timing has a duration), this can be specified to convey the total amount to be administered over the period of time as indicated by the schedule e.g. 500 ml in dose, with timing used to convey that this should be done over 4 hours. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub dose: Option<DosageDoseAndRateDose>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub dose_ext: Option<DosageDoseAndRateDoseExtension>,
    /** **Amount of medication per unit of time**

 Amount of medication per unit of time.

 It is possible to supply both a rate and a doseQuantity to provide full details about how the medication is to be administered and supplied. If the rate is intended to change over time, depending on local rules/regulations, each change should be captured as a new version of the MedicationRequest with an updated rate, or captured with a new MedicationRequest with the new rate.

It is possible to specify a rate over time (for example, 100 ml/hour) using either the rateRatio and rateQuantity.  The rateQuantity approach requires systems to have the capability to parse UCUM grammar where ml/hour is included rather than a specific ratio where the time is specified as the denominator.  Where a rate such as 500ml over 2 hours is specified, the use of rateRatio may be more semantically correct than specifying using a rateQuantity of 250 mg/hour. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub rate: Option<DosageDoseAndRateRate>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub rate_ext: Option<DosageDoseAndRateRateExtension>,
}
#[cfg(feature = "builders")]
impl DosageDoseAndRate {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DosageDoseAndRateBuilder {
        DosageDoseAndRateBuilder::default()
    }
}
/// Choice of types for the dose[x] field in DosageDoseAndRate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageDoseAndRateDose {
    /// Variant accepting the Range type.
    #[serde(rename = "doseRange")]
    Range(Range),
    /// Variant accepting the Quantity type.
    #[serde(rename = "doseQuantity")]
    Quantity(Quantity),
}
/// Extension value for DosageDoseAndRateDose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageDoseAndRateDoseExtension {
    /// Variant accepting the Range extension.
    #[serde(rename = "_doseRange")]
    Range(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_doseQuantity")]
    Quantity(FieldExtension),
}
/// Choice of types for the rate[x] field in DosageDoseAndRate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageDoseAndRateRate {
    /// Variant accepting the Ratio type.
    #[serde(rename = "rateRatio")]
    Ratio(Ratio),
    /// Variant accepting the Range type.
    #[serde(rename = "rateRange")]
    Range(Range),
    /// Variant accepting the Quantity type.
    #[serde(rename = "rateQuantity")]
    Quantity(Quantity),
}
/// Extension value for DosageDoseAndRateRate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageDoseAndRateRateExtension {
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_rateRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_rateRange")]
    Range(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_rateQuantity")]
    Quantity(FieldExtension),
}
/** Duration Type: A length of time.

 **[Duration](http://hl7.org/fhir/StructureDefinition/Duration) v5.0.0**

 A length of time

 A length of time.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Duration(pub Box<DurationInner>);
/** Duration Type: A length of time.

 **[Duration](http://hl7.org/fhir/StructureDefinition/Duration) v5.0.0**

 A length of time

 A length of time.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DurationBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DurationInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl DurationBuilder {
    #[doc = concat!("Finalize building ", "Duration", ".")]
    pub fn build(self) -> Result<Duration, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DurationInner> for Duration {
    fn from(inner: DurationInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Duration {
    type Target = DurationInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Duration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Duration {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DurationBuilder {
        DurationBuilder::default()
    }
}
/** ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.

 **[ElementDefinition](http://hl7.org/fhir/StructureDefinition/ElementDefinition) v5.0.0**

 Definition of an element in a resource or extension

 Captures constraints on each element within the resource, profile, or extension.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ElementDefinition(pub Box<ElementDefinitionInner>);
/** ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.

 **[ElementDefinition](http://hl7.org/fhir/StructureDefinition/ElementDefinition) v5.0.0**

 Definition of an element in a resource or extension

 Captures constraints on each element within the resource, profile, or extension.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ElementDefinitionInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Extensions that cannot be ignored even if unrecognized**

 May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.

Modifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself).

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Vec<Extension>,
    /** **Path of the element in the hierarchy of elements**

 The path identifies the element and is expressed as a "."-separated list of ancestor elements, beginning with the name of the resource or extension.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **[PropertyRepresentation](http://hl7.org/fhir/ValueSet/property-representation); xmlAttr | xmlText | typeAttr | cdaText | xhtml**

 Codes that define how this element is represented in instances, when the deviation varies from the normal case. No extensions are allowed on elements with a representation of 'xmlAttr', no matter what FHIR serialization format is used.

 In resources, this is rarely used except for special cases where the representation deviates from the normal, and can only be done in the base standard (and profiles must reproduce what the base standard does). This element is used quite commonly in Logical models when the logical models represent a specific serialization format (e.g. CDA, v2 etc.). */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "representation")]
    pub representation: Vec<Option<codes::PropertyRepresentation>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_representation")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub representation_ext: Vec<Option<FieldExtension>>,
    /** **Name for this particular element (in a set of slices)**

 The name of this element definition slice, when slicing is working. The name must be a token with no dots or spaces. This is a unique name referring to a specific set of constraints applied to this element, used to provide a name to different slices of the same element.

 The name SHALL be unique within the structure within the context of the constrained resource element.  (Though to avoid confusion, uniqueness across all elements is recommended.). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sliceName")]
    pub slice_name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sliceName")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub slice_name_ext: Option<FieldExtension>,
    /** **If this slice definition constrains an inherited slice definition (or not)**

 If true, indicates that this slice definition is constraining a slice definition with the same name in an inherited profile. If false, the slice is not overriding any slice in an inherited profile. If missing, the slice might or might not be overriding a slice in an inherited profile, depending on the sliceName.

 If set to true, an ancestor profile SHALL have a slicing definition with this name.  If set to false, no ancestor profile is permitted to have a slicing definition with this name. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sliceIsConstraining")]
    pub slice_is_constraining: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sliceIsConstraining")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub slice_is_constraining_ext: Option<FieldExtension>,
    /** **Name for element to display with or prompt for element**

 A single preferred label which is the text to display beside the element indicating its meaning or to use to prompt for the element in a user display or form.

 See also the extension [http://hl7.org/fhir/StructureDefinition/elementdefinition-question](http://hl7.org/fhir/extensions/StructureDefinition-elementdefinition-question.html). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_label")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub label_ext: Option<FieldExtension>,
    /** **[ElementDefinitionCode](http://loinc.org/vs); Corresponding codes in terminologies**

 A code that has the same meaning as the element in a particular terminology.

 The concept SHALL be properly aligned with the data element definition and other constraints, as defined in the code system, including relationships, of any code listed here.  Where multiple codes exist in a terminology that could correspond to the data element, the most granular code(s) should be selected, so long as they are not more restrictive than the data element itself. The mappings may be used to provide more or less granular or structured equivalences in the code system. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub code_ext: Vec<Option<FieldExtension>>,
    /** **This element is sliced - slices follow**

 Indicates that the element is sliced into a set of alternative definitions (i.e. in a structure definition, there are multiple different constraints on a single element in the base resource). Slicing can be used in any resource that has cardinality ..* on the base resource, or any resource with a choice of types. The set of slices is any elements that come after this in the element sequence that have the same path, until a shorter path occurs (the shorter path terminates the set).

 The first element in the sequence, the one that carries the slicing, is the definition that applies to all the slices. This is based on the unconstrained element, but can apply any constraints as appropriate. This may include the common constraints on the children of the element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "slicing")]
    pub slicing: Option<ElementDefinitionSlicing>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_slicing")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub slicing_ext: Option<FieldExtension>,
    /** **Concise definition for space-constrained presentation**

 A concise description of what this element means (e.g. for use in autogenerated summaries).

 It is easy for a different short definition to change the meaning of an element and this can have nasty downstream consequences. Please be careful when providing short definitions in a profile. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "short")]
    pub short: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_short")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub short_ext: Option<FieldExtension>,
    /** **Full formal definition as narrative text**

 Provides a complete explanation of the meaning of the data element for human readability.  For the case of elements derived from existing elements (e.g. constraints), the definition SHALL be consistent with the base definition, but convey the meaning of the element in the particular context of use of the resource. (Note: The text you are reading is specified in ElementDefinition.definition).

 It is easy for a different definition to change the meaning of an element and this can have nasty downstream consequences. Please be careful when providing definitions in a profile. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "definition")]
    pub definition: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_definition")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub definition_ext: Option<FieldExtension>,
    /** **Comments about the use of this element**

 Explanatory notes and implementation guidance about the data element, including notes about how to use the data properly, exceptions to proper use, etc. (Note: The text you are reading is specified in ElementDefinition.comment).

 If it is possible to capture usage rules using constraints, that mechanism should be used in preference to this element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comment")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comment_ext: Option<FieldExtension>,
    /** **Why this resource has been created**

 This element is for traceability of why the element was created and why the constraints exist as they do. This may be used to point to source materials or specifications that drove the structure of this element.

 This element does not describe the usage of the element (that's done in comments), rather it's for traceability of *why* the element is either needed or why the constraints exist as they do.  This may be used to point to source materials or specifications that drove the structure of this data element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "requirements")]
    pub requirements: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_requirements")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub requirements_ext: Option<FieldExtension>,
    /** **Other names**

 Identifies additional names by which this element might also be known.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "alias")]
    pub alias: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_alias")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub alias_ext: Vec<Option<FieldExtension>>,
    /** **Minimum Cardinality**

 The minimum number of times this element SHALL appear in the instance.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "min")]
    pub min: Option<u32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_min")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_ext: Option<FieldExtension>,
    /** **Maximum Cardinality (a number or *)**

 The maximum number of times this element is permitted to appear in the instance.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_max")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_ext: Option<FieldExtension>,
    /** **Base definition information for tools**

 Information about the base definition of the element, provided to make it unnecessary for tools to trace the deviation of the element through the derived and related profiles. When the element definition is not the original definition of an element - e.g. either in a constraint on another type, or for elements from a super type in a snap shot - then the information in provided in the element definition may be different to the base definition. On the original definition of the element, it will be same.

 The base information does not carry any information that could not be determined from the path and related profiles, but making this determination requires both that the related profiles are available, and that the algorithm to determine them be available. For tooling simplicity, the base information must always be populated in element definitions in snap shots, even if it is the same. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "base")]
    pub base: Option<ElementDefinitionBase>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_base")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub base_ext: Option<FieldExtension>,
    /** **Reference to definition of content for the element**

 Identifies an element defined elsewhere in the definition whose content rules should be applied to the current element. ContentReferences bring across all the rules that are in the ElementDefinition for the element, including definitions, cardinality constraints, bindings, invariants etc.

 ContentReferences can only be defined in specializations, not constrained types, and they cannot be changed and always reference the non-constrained definition. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contentReference")]
    pub content_reference: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_contentReference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub content_reference_ext: Option<FieldExtension>,
    /** **Data type and Profile for this element**

 The data type or resource that the value of this element is permitted to be.

 The Type of the element can be left blank in a differential constraint, in which case the type is inherited from the resource. Abstract types are not permitted to appear as a type when multiple types are listed.  (I.e. Abstract types cannot be part of a choice). */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Vec<Option<ElementDefinitionType>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub r#type_ext: Vec<Option<FieldExtension>>,
    /** **Specified value if missing from instance**

 The value that should be used if there is no value stated in the instance (e.g. 'if not otherwise specified, the abstract is false').

 Specifying a default value means that the property can never been unknown - it must always have a value. Further, the default value can never be changed, or changed in constraints on content models. Defining default values creates many difficulties in implementation (e.g. when is a value missing?). For these reasons, default values are (and should be) used extremely sparingly.

No default values are ever defined in the FHIR specification, nor can they be defined in constraints ("profiles") on data types or resources. This element only exists so that default values may be defined in logical models. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub default_value: Option<ElementDefinitionDefaultValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub default_value_ext: Option<ElementDefinitionDefaultValueExtension>,
    /** **Implicit meaning when this element is missing**

 The Implicit meaning that is to be understood when this element is missing (e.g. 'when this element is missing, the period is ongoing').

 Implicit meanings for missing values can only be specified on a resource, data type, or extension definition, and never in a profile that applies to one of these. An implicit meaning for a missing value can never be changed, and specifying one has the consequence that constraining its use in profiles eliminates use cases as possibilities, not merely moving them out of scope. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "meaningWhenMissing")]
    pub meaning_when_missing: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_meaningWhenMissing")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub meaning_when_missing_ext: Option<FieldExtension>,
    /** **What the order of the elements means**

 If present, indicates that the order of the repeating element has meaning and describes what that meaning is.  If absent, it means that the order of the element has no meaning.

 This element can only be asserted on repeating elements and can only be introduced when defining resources or data types.  It can be further refined profiled elements but if absent in the base type, a profile cannot assert meaning. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "orderMeaning")]
    pub order_meaning: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_orderMeaning")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub order_meaning_ext: Option<FieldExtension>,
    /** **Value must be exactly this**

 Specifies a value that SHALL be exactly the value  for this element in the instance, if present. For purposes of comparison, non-significant whitespace is ignored, and all values must be an exact match (case and accent sensitive). Missing elements/attributes must also be missing.

 This is not recommended for Coding and CodeableConcept since these often have highly contextual properties such as version or display. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub fixed: Option<ElementDefinitionFixed>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub fixed_ext: Option<ElementDefinitionFixedExtension>,
    /** **Value must have at least these property values**

 Specifies a value that each occurrence of the element in the instance SHALL follow - that is, any value in the pattern must be found in the instance, if the element has a value. Other additional values may be found too. This is effectively constraint by example.

When pattern[x] is used to constrain a primitive, it means that the value provided in the pattern[x] must match the instance value exactly.

When an element within a pattern[x] is used to constrain an array, it means that each element provided in the pattern[x] must (recursively) match at least one element from the instance array.

When pattern[x] is used to constrain a complex object, it means that each property in the pattern must be present in the complex object, and its value must recursively match -- i.e.,

1. If primitive: it must match exactly the pattern value
2. If a complex object: it must match (recursively) the pattern value
3. If an array: it must match (recursively) the pattern value

If a pattern[x] is declared on a repeating element, the pattern applies to all repetitions.  If the desire is for a pattern to apply to only one element or a subset of elements, slicing must be used. See [Examples of Patterns](elementdefinition-examples.html#pattern-examples) for examples of pattern usage and the effect it will have.

 Mostly used for fixing values of CodeableConcept. In general, pattern[x] is not intended for use with primitive types, where is has the same meaning as fixed[x]. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub pattern: Option<ElementDefinitionPattern>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub pattern_ext: Option<ElementDefinitionPatternExtension>,
    /** **Example value (as defined for type)**

 A sample value for this element demonstrating the type of information that would typically be found in the element.

 Examples will most commonly be present for data where it's not implicitly obvious from either the data type or value set what the values might be.  (I.e. Example values for dates or quantities would generally be unnecessary.)  If the example value is fully populated, the publication tool can generate an instance automatically. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "example")]
    pub example: Vec<Option<ElementDefinitionExample>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_example")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub example_ext: Vec<Option<FieldExtension>>,
    /** **Minimum Allowed Value (for some types)**

 The minimum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity.

 Except for date/date/instant, the type of the minValue[x] SHALL be the same as the specified type of the element. For the date/dateTime/instant values, the type of minValue[x] SHALL be either the same, or a [Duration](datatypes.html#Duration) which specifies a relative time limit to the current time. The duration value is positive, and is subtracted from the current clock to determine the minimum allowable value.   A minimum value for a Quantity is interpreted as a canonical minimum - e.g. you cannot provide 100mg if the minimum value is 10g. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub min_value: Option<ElementDefinitionMinValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_value_ext: Option<ElementDefinitionMinValueExtension>,
    /** **Maximum Allowed Value (for some types)**

 The maximum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity.

 Except for date/date/instant, the type of the maxValue[x] SHALL be the same as the specified type of the element. For the date/dateTime/instant values, the type of maxValue[x] SHALL be either the same, or a [Duration](datatypes.html#Duration) which specifies a relative time limit to the current time. The duration value is positive, and is added to the current clock to determine the maximum allowable value.   A maximum value for a Quantity is interpreted as a canonical maximum - e.g. you cannot provide 10g if the maximum value is 50mg. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub max_value: Option<ElementDefinitionMaxValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_value_ext: Option<ElementDefinitionMaxValueExtension>,
    /** **Max length for string type data**

 Indicates the maximum length in characters that is permitted to be present in conformant instances and which is expected to be supported by conformant consumers that support the element. ```textmaxLength``` SHOULD only be used on primitive data types that have a string representation (see [http://hl7.org/fhir/StructureDefinition/structuredefinition-type-characteristics](http://hl7.org/fhir/extensions/StructureDefinition-structuredefinition-type-characteristics.html)).

 Receivers are not required to reject instances that exceed the maximum length.  The full length could be stored.  In some cases, data might be truncated, though truncation should be undertaken with care and an understanding of the consequences of doing so. If not specified, there is no conformance expectation for length support. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxLength")]
    pub max_length: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxLength")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_length_ext: Option<FieldExtension>,
    /** **Reference to invariant about presence**

 A reference to an invariant that may make additional statements about the cardinality or value in the instance.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "condition")]
    pub condition: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_condition")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub condition_ext: Vec<Option<FieldExtension>>,
    /** **Condition that must evaluate to true**

 Formal constraints such as co-occurrence and other constraints that can be computationally evaluated within the context of the instance.

 Constraints should be declared on the "context" element - the lowest element in the hierarchy that is common to all nodes referenced by the constraint. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "constraint")]
    pub constraint: Vec<Option<ElementDefinitionConstraint>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_constraint")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub constraint_ext: Vec<Option<FieldExtension>>,
    /** **For primitives, that a value must be present - not replaced by an extension**

 Specifies for a primitive data type that the value of the data type cannot be replaced by an extension.

 Typical extensions that can be provided in place of a the value are [http://hl7.org/fhir/StructureDefinition/data-absent-reason](http://hl7.org/fhir/extensions/StructureDefinition-data-absent-reason.html),  [http://hl7.org/fhir/StructureDefinition/iso21090-nullFlavor](http://hl7.org/fhir/extensions/StructureDefinition-iso21090-nullFlavor.html),  [http://hl7.org/fhir/StructureDefinition/originalText](http://hl7.org/fhir/extensions/StructureDefinition-originalText.html), and [http://hl7.org/fhir/StructureDefinition/cqf-expression](http://hl7.org/fhir/extensions/StructureDefinition-cqf-expression.html). Note that this element has no meaning when a value is provided, and extensions can be provided in addition to a value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mustHaveValue")]
    pub must_have_value: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_mustHaveValue")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub must_have_value_ext: Option<FieldExtension>,
    /** **Extensions that are allowed to replace a primitive value**

 Specifies a list of extensions that can appear in place of a primitive value.

 if mustHaveValue is true, then this element should not be present, since no extensions are allowed in place of the value. Note that this element has no impact if the value is present. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueAlternatives")]
    pub value_alternatives: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_valueAlternatives")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub value_alternatives_ext: Vec<Option<FieldExtension>>,
    /** **If the element must be supported (discouraged - see obligations)**

 If true, implementations that produce or consume resources SHALL provide "support" for the element in some meaningful way. Note that this is being phased out and replaced by obligations (see below).  If false, the element may be ignored and not supported. If false, whether to populate or use the data element in any way is at the discretion of the implementation.

 "Something meaningful" is context dependent and impossible to describe in the base FHIR specification. For this reason, the mustSupport flag is never set to true by the FHIR specification itself - it is only set to true in profiles.  A profile on a type can always make mustSupport = true if it is false in the base type but cannot make mustSupport = false if it is true in the base type.   This is done in [Resource Profiles](profiling.html#mustsupport), where the profile labels an element as mustSupport=true. When a profile does this, it SHALL also make clear exactly what kind of "support" is required, as this can mean many things.    Note that an element that has the property IsModifier is not necessarily a "key" element (e.g. one of the important elements to make use of the resource), nor is it automatically mustSupport - however both of these things are more likely to be true for IsModifier elements than for other elements. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mustSupport")]
    pub must_support: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_mustSupport")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub must_support_ext: Option<FieldExtension>,
    /** **If this modifies the meaning of other elements**

 If true, the value of this element affects the interpretation of the element or resource that contains it, and the value of the element cannot be ignored. Typically, this is used for status, negation and qualification codes. The effect of this is that the element cannot be ignored by systems: they SHALL either recognize the element and process it, and/or a pre-determination has been made that it is not relevant to their particular system. When used on the root element in an extension definition, this indicates whether or not the extension is a modifier extension.

 Only the definition of an element can set IsModifier true - either the specification itself or where an extension is originally defined. Once set, it cannot be changed in derived profiles (except in the special case of the defining a new extension). An element/extension that has isModifier=true SHOULD also have a minimum cardinality of 1, so that there is no lack of clarity about what to do if it is missing. If it can be missing, the definition SHALL make the meaning of a missing element clear. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "isModifier")]
    pub is_modifier: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_isModifier")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub is_modifier_ext: Option<FieldExtension>,
    /** **Reason that this element is marked as a modifier**

 Explains how that element affects the interpretation of the resource or element that contains it.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "isModifierReason")]
    pub is_modifier_reason: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_isModifierReason")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub is_modifier_reason_ext: Option<FieldExtension>,
    /** **Include when _summary = true?**

 Whether the element should be included if a client requests a search with the parameter _summary=true.

 Some resources include a set of simple metadata, and some very large data. This element is used to reduce the quantity of data returned in searches. Note that servers may pre-cache summarized resources for optimal performance. When a request is made with _summary=true, serializers only include elements marked as 'isSummary = true'. With a few exceptions (listed below), all datatype properties are included in the summary form. In resource and datatype definitions, if an element has a descendant marked as isSummary=true or if it satisfies both of the following conditions, it must be marked as isSummary=true:
* is at the root or has a parent that is 'mustSupport'
* has a minimum cardinality of 1 or is a modifier element

The following datatype properties are exceptions, and are not marked as isSummary:
* Attachment.data
* Signature.data, Signature.targetFormat, Signature.sigFormat
* Narrative.status, Narrative.div
* SampledData.data. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "isSummary")]
    pub is_summary: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_isSummary")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub is_summary_ext: Option<FieldExtension>,
    /** **ValueSet details if this is coded**

 Binds to a value set if this element is coded (code, Coding, CodeableConcept, Quantity), or the data types (string, uri).

 For a CodeableConcept, when no codes are allowed - only text, use a binding of strength "required" with a description explaining that no coded values are allowed and what sort of information to put in the "text" element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "binding")]
    pub binding: Option<ElementDefinitionBinding>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_binding")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub binding_ext: Option<FieldExtension>,
    /** **Map element to another set of definitions**

 Identifies a concept from an external specification that roughly corresponds to this element.

 Mappings are not necessarily specific enough for safe translation. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mapping")]
    pub mapping: Vec<Option<ElementDefinitionMapping>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_mapping")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub mapping_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBuilder {
    #[doc = concat!("Finalize building ", "ElementDefinition", ".")]
    pub fn build(self) -> Result<ElementDefinition, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ElementDefinitionInner> for ElementDefinition {
    fn from(inner: ElementDefinitionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ElementDefinition {
    type Target = ElementDefinitionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ElementDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ElementDefinition {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ElementDefinitionBuilder {
        ElementDefinitionBuilder::default()
    }
}
/// Sub-fields of the slicing field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionSlicingBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionSlicing {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Element values that are used to distinguish the slices**

 Designates which child elements are used to discriminate between the slices when processing an instance. If one or more discriminators are provided, the value of the child elements in the instance data SHALL completely distinguish which slice the element in the resource matches based on the allowed values for those elements in each of the slices.

 If there is no discriminator, the content is hard to process, so this should be avoided. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "discriminator")]
    pub discriminator: Vec<Option<ElementDefinitionSlicingDiscriminator>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_discriminator")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub discriminator_ext: Vec<Option<FieldExtension>>,
    /** **Text description of how slicing works (or not)**

 A human-readable text description of how the slicing works. If there is no discriminator, this is required to be present to provide whatever information is possible about how the slices can be differentiated.

 If it's really not possible to differentiate them, the design should be re-evaluated to make the content usable. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_description")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub description_ext: Option<FieldExtension>,
    /** **If elements must be in same order as slices**

 If the matching elements have to occur in the same order as defined in the profile.

 Order should only be required when it is a pressing concern for presentation. Profile authors should consider making the order a feature of the rules about the narrative, not the rules about the data - requiring ordered data makes the profile much less re-usable. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "ordered")]
    pub ordered: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_ordered")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub ordered_ext: Option<FieldExtension>,
    /** **[SlicingRules](http://hl7.org/fhir/ValueSet/resource-slicing-rules); closed | open | openAtEnd**

 Whether additional slices are allowed or not. When the slices are ordered, profile authors can also say that additional slices are only allowed at the end.

 Allowing additional elements makes for a much for flexible template - it's open for use in wider contexts, but also means that the content of the resource is not closed, and applications have to decide how to handle content not described by the profile. */
    #[serde(rename = "rules")]
    pub rules: codes::SlicingRules,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_rules")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub rules_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionSlicing {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionSlicingBuilder {
        ElementDefinitionSlicingBuilder::default()
    }
}
/// Sub-fields of the discriminator field in ElementDefinitionSlicing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionSlicingDiscriminatorBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionSlicingDiscriminator {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[DiscriminatorType](http://hl7.org/fhir/ValueSet/discriminator-type); value | exists | type | profile | position**

 How the element value is interpreted when discrimination is evaluated.

 'pattern' is deprecated - it works exactly the same as 'value'. */
    #[serde(rename = "type")]
    pub r#type: codes::DiscriminatorType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Path to element value**

 A FHIRPath expression, using [the simple subset of FHIRPath](fhirpath.html#simple), that is used to identify the element on which discrimination is based.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionSlicingDiscriminator {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionSlicingDiscriminatorBuilder {
        ElementDefinitionSlicingDiscriminatorBuilder::default()
    }
}
/// Sub-fields of the base field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBaseBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionBase {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Path that identifies the base element**

 The Path that identifies the base element - this matches the ElementDefinition.path for that element. Across FHIR, there is only one base definition of any element - that is, an element definition on a [StructureDefinition](structuredefinition.html#) without a StructureDefinition.base.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **Min cardinality of the base element**

 Minimum cardinality of the base element identified by the path.

 This is provided for consistency with max, and may affect code generation of mandatory elements of the base resource are generated differently (some reference implementations have done this). */
    #[serde(rename = "min")]
    pub min: u32,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_min")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_ext: Option<FieldExtension>,
    /** **Max cardinality of the base element**

 Maximum cardinality of the base element identified by the path.

 This is provided to code generation, since the serialization representation in JSON differs depending on whether the base element has max > 1. Also, some forms of code generation may differ. */
    #[serde(rename = "max")]
    pub max: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_max")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBase {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionBaseBuilder {
        ElementDefinitionBaseBuilder::default()
    }
}
/// Sub-fields of the type field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionTypeBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionType {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[ElementDefinitionTypes](http://hl7.org/fhir/ValueSet/elementdefinition-types); Data type or Resource (reference to definition)**

 URL of Data type or Resource that is a(or the) type used for this element. References are URLs that are relative to http://hl7.org/fhir/StructureDefinition e.g. "string" is a reference to http://hl7.org/fhir/StructureDefinition/string. Absolute URLs are only allowed in logical models.

 If the element is a reference to another resource, this element contains "Reference", and the targetProfile element defines what resources can be referenced. The targetProfile may be a reference to the general definition of a resource (e.g. http://hl7.org/fhir/StructureDefinition/Patient). */
    #[serde(rename = "code")]
    pub code: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Profiles (StructureDefinition or IG) - one must apply**

 Identifies a profile structure or implementation Guide that applies to the datatype this element refers to. If any profiles are specified, then the content must conform to at least one of them. The URL can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the type SHALL conform to at least one profile defined in the implementation guide.

 It is possible to profile  backbone element (e.g. part of a resource), using the [http://hl7.org/fhir/StructureDefinition/elementdefinition-profile-element](http://hl7.org/fhir/extensions/StructureDefinition-elementdefinition-profile-element.html) extension. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub profile_ext: Vec<Option<FieldExtension>>,
    /** **Profile (StructureDefinition or IG) on the Reference/canonical target - one must apply**

 Used when the type is "Reference" or "canonical", and identifies a profile structure or implementation Guide that applies to the target of the reference this element refers to. If any profiles are specified, then the content must conform to at least one of them. The URL can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the target resource SHALL conform to at least one profile defined in the implementation guide.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "targetProfile")]
    pub target_profile: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_targetProfile")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub target_profile_ext: Vec<Option<FieldExtension>>,
    /** **[AggregationMode](http://hl7.org/fhir/ValueSet/resource-aggregation-mode); contained | referenced | bundled - how aggregated**

 If the type is a reference to another resource, how the resource is or can be aggregated - is it a contained resource, or a reference, and if the context is a bundle, is it included in the bundle.

 See [Aggregation Rules](elementdefinition.html#aggregation) for further clarification. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "aggregation")]
    pub aggregation: Vec<Option<codes::AggregationMode>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_aggregation")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub aggregation_ext: Vec<Option<FieldExtension>>,
    /** **[ReferenceVersionRules](http://hl7.org/fhir/ValueSet/reference-version-rules); either | independent | specific**

 Whether this reference needs to be version specific or version independent, or whether either can be used.

 The base specification never makes a rule as to which form is allowed, but implementation guides may do this. See [Aggregation Rules](elementdefinition.html#aggregation) for further clarification. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "versioning")]
    pub versioning: Option<codes::ReferenceVersionRules>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_versioning")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub versioning_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionType {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionTypeBuilder {
        ElementDefinitionTypeBuilder::default()
    }
}
/// Choice of types for the defaultValue[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionDefaultValue {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "defaultValueBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "defaultValueBoolean")]
    Boolean(bool),
    /// Variant accepting the Canonical type.
    #[serde(rename = "defaultValueCanonical")]
    Canonical(String),
    /// Variant accepting the Code type.
    #[serde(rename = "defaultValueCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "defaultValueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "defaultValueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "defaultValueDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "defaultValueId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "defaultValueInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "defaultValueInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "defaultValueInteger64")]
    Integer64(Integer64),
    /// Variant accepting the Markdown type.
    #[serde(rename = "defaultValueMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "defaultValueOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "defaultValuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "defaultValueString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "defaultValueTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "defaultValueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "defaultValueUri")]
    Uri(String),
    /// Variant accepting the Url type.
    #[serde(rename = "defaultValueUrl")]
    Url(String),
    /// Variant accepting the Uuid type.
    #[serde(rename = "defaultValueUuid")]
    Uuid(String),
    /// Variant accepting the Address type.
    #[serde(rename = "defaultValueAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "defaultValueAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "defaultValueAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "defaultValueAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "defaultValueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the CodeableReference type.
    #[serde(rename = "defaultValueCodeableReference")]
    CodeableReference(CodeableReference),
    /// Variant accepting the Coding type.
    #[serde(rename = "defaultValueCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "defaultValueContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "defaultValueCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "defaultValueDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "defaultValueDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "defaultValueHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "defaultValueIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "defaultValueMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "defaultValuePeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "defaultValueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "defaultValueRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "defaultValueRatio")]
    Ratio(Ratio),
    /// Variant accepting the RatioRange type.
    #[serde(rename = "defaultValueRatioRange")]
    RatioRange(RatioRange),
    /// Variant accepting the Reference type.
    #[serde(rename = "defaultValueReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "defaultValueSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "defaultValueSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "defaultValueTiming")]
    Timing(Timing),
    /// Variant accepting the ContactDetail type.
    #[serde(rename = "defaultValueContactDetail")]
    ContactDetail(ContactDetail),
    /// Variant accepting the DataRequirement type.
    #[serde(rename = "defaultValueDataRequirement")]
    DataRequirement(DataRequirement),
    /// Variant accepting the Expression type.
    #[serde(rename = "defaultValueExpression")]
    Expression(Expression),
    /// Variant accepting the ParameterDefinition type.
    #[serde(rename = "defaultValueParameterDefinition")]
    ParameterDefinition(ParameterDefinition),
    /// Variant accepting the RelatedArtifact type.
    #[serde(rename = "defaultValueRelatedArtifact")]
    RelatedArtifact(RelatedArtifact),
    /// Variant accepting the TriggerDefinition type.
    #[serde(rename = "defaultValueTriggerDefinition")]
    TriggerDefinition(TriggerDefinition),
    /// Variant accepting the UsageContext type.
    #[serde(rename = "defaultValueUsageContext")]
    UsageContext(UsageContext),
    /// Variant accepting the Availability type.
    #[serde(rename = "defaultValueAvailability")]
    Availability(Availability),
    /// Variant accepting the ExtendedContactDetail type.
    #[serde(rename = "defaultValueExtendedContactDetail")]
    ExtendedContactDetail(ExtendedContactDetail),
    /// Variant accepting the Dosage type.
    #[serde(rename = "defaultValueDosage")]
    Dosage(Dosage),
    /// Variant accepting the Meta type.
    #[serde(rename = "defaultValueMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionDefaultValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionDefaultValueExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_defaultValueBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_defaultValueBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Canonical extension.
    #[serde(rename = "_defaultValueCanonical")]
    Canonical(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_defaultValueCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_defaultValueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_defaultValueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_defaultValueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_defaultValueId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_defaultValueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_defaultValueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_defaultValueInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_defaultValueMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_defaultValueOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_defaultValuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_defaultValueString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_defaultValueTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_defaultValueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_defaultValueUri")]
    Uri(FieldExtension),
    /// Variant accepting the Url extension.
    #[serde(rename = "_defaultValueUrl")]
    Url(FieldExtension),
    /// Variant accepting the Uuid extension.
    #[serde(rename = "_defaultValueUuid")]
    Uuid(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_defaultValueAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_defaultValueAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_defaultValueAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_defaultValueAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_defaultValueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the CodeableReference extension.
    #[serde(rename = "_defaultValueCodeableReference")]
    CodeableReference(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_defaultValueCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_defaultValueContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_defaultValueCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_defaultValueDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_defaultValueDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_defaultValueHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_defaultValueIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_defaultValueMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_defaultValuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_defaultValueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_defaultValueRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_defaultValueRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the RatioRange extension.
    #[serde(rename = "_defaultValueRatioRange")]
    RatioRange(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_defaultValueReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_defaultValueSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_defaultValueSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_defaultValueTiming")]
    Timing(FieldExtension),
    /// Variant accepting the ContactDetail extension.
    #[serde(rename = "_defaultValueContactDetail")]
    ContactDetail(FieldExtension),
    /// Variant accepting the DataRequirement extension.
    #[serde(rename = "_defaultValueDataRequirement")]
    DataRequirement(FieldExtension),
    /// Variant accepting the Expression extension.
    #[serde(rename = "_defaultValueExpression")]
    Expression(FieldExtension),
    /// Variant accepting the ParameterDefinition extension.
    #[serde(rename = "_defaultValueParameterDefinition")]
    ParameterDefinition(FieldExtension),
    /// Variant accepting the RelatedArtifact extension.
    #[serde(rename = "_defaultValueRelatedArtifact")]
    RelatedArtifact(FieldExtension),
    /// Variant accepting the TriggerDefinition extension.
    #[serde(rename = "_defaultValueTriggerDefinition")]
    TriggerDefinition(FieldExtension),
    /// Variant accepting the UsageContext extension.
    #[serde(rename = "_defaultValueUsageContext")]
    UsageContext(FieldExtension),
    /// Variant accepting the Availability extension.
    #[serde(rename = "_defaultValueAvailability")]
    Availability(FieldExtension),
    /// Variant accepting the ExtendedContactDetail extension.
    #[serde(rename = "_defaultValueExtendedContactDetail")]
    ExtendedContactDetail(FieldExtension),
    /// Variant accepting the Dosage extension.
    #[serde(rename = "_defaultValueDosage")]
    Dosage(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_defaultValueMeta")]
    Meta(FieldExtension),
}
/// Choice of types for the fixed[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionFixed {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "fixedBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "fixedBoolean")]
    Boolean(bool),
    /// Variant accepting the Canonical type.
    #[serde(rename = "fixedCanonical")]
    Canonical(String),
    /// Variant accepting the Code type.
    #[serde(rename = "fixedCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "fixedDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "fixedDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "fixedDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "fixedId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "fixedInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "fixedInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "fixedInteger64")]
    Integer64(Integer64),
    /// Variant accepting the Markdown type.
    #[serde(rename = "fixedMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "fixedOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "fixedPositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "fixedString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "fixedTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "fixedUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "fixedUri")]
    Uri(String),
    /// Variant accepting the Url type.
    #[serde(rename = "fixedUrl")]
    Url(String),
    /// Variant accepting the Uuid type.
    #[serde(rename = "fixedUuid")]
    Uuid(String),
    /// Variant accepting the Address type.
    #[serde(rename = "fixedAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "fixedAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "fixedAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "fixedAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "fixedCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the CodeableReference type.
    #[serde(rename = "fixedCodeableReference")]
    CodeableReference(CodeableReference),
    /// Variant accepting the Coding type.
    #[serde(rename = "fixedCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "fixedContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "fixedCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "fixedDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "fixedDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "fixedHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "fixedIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "fixedMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "fixedPeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "fixedQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "fixedRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "fixedRatio")]
    Ratio(Ratio),
    /// Variant accepting the RatioRange type.
    #[serde(rename = "fixedRatioRange")]
    RatioRange(RatioRange),
    /// Variant accepting the Reference type.
    #[serde(rename = "fixedReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "fixedSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "fixedSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "fixedTiming")]
    Timing(Timing),
    /// Variant accepting the ContactDetail type.
    #[serde(rename = "fixedContactDetail")]
    ContactDetail(ContactDetail),
    /// Variant accepting the DataRequirement type.
    #[serde(rename = "fixedDataRequirement")]
    DataRequirement(DataRequirement),
    /// Variant accepting the Expression type.
    #[serde(rename = "fixedExpression")]
    Expression(Expression),
    /// Variant accepting the ParameterDefinition type.
    #[serde(rename = "fixedParameterDefinition")]
    ParameterDefinition(ParameterDefinition),
    /// Variant accepting the RelatedArtifact type.
    #[serde(rename = "fixedRelatedArtifact")]
    RelatedArtifact(RelatedArtifact),
    /// Variant accepting the TriggerDefinition type.
    #[serde(rename = "fixedTriggerDefinition")]
    TriggerDefinition(TriggerDefinition),
    /// Variant accepting the UsageContext type.
    #[serde(rename = "fixedUsageContext")]
    UsageContext(UsageContext),
    /// Variant accepting the Availability type.
    #[serde(rename = "fixedAvailability")]
    Availability(Availability),
    /// Variant accepting the ExtendedContactDetail type.
    #[serde(rename = "fixedExtendedContactDetail")]
    ExtendedContactDetail(ExtendedContactDetail),
    /// Variant accepting the Dosage type.
    #[serde(rename = "fixedDosage")]
    Dosage(Dosage),
    /// Variant accepting the Meta type.
    #[serde(rename = "fixedMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionFixed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionFixedExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_fixedBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_fixedBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Canonical extension.
    #[serde(rename = "_fixedCanonical")]
    Canonical(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_fixedCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_fixedDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_fixedDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_fixedDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_fixedId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_fixedInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_fixedInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_fixedInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_fixedMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_fixedOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_fixedPositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_fixedString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_fixedTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_fixedUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_fixedUri")]
    Uri(FieldExtension),
    /// Variant accepting the Url extension.
    #[serde(rename = "_fixedUrl")]
    Url(FieldExtension),
    /// Variant accepting the Uuid extension.
    #[serde(rename = "_fixedUuid")]
    Uuid(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_fixedAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_fixedAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_fixedAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_fixedAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_fixedCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the CodeableReference extension.
    #[serde(rename = "_fixedCodeableReference")]
    CodeableReference(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_fixedCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_fixedContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_fixedCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_fixedDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_fixedDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_fixedHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_fixedIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_fixedMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_fixedPeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_fixedQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_fixedRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_fixedRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the RatioRange extension.
    #[serde(rename = "_fixedRatioRange")]
    RatioRange(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_fixedReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_fixedSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_fixedSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_fixedTiming")]
    Timing(FieldExtension),
    /// Variant accepting the ContactDetail extension.
    #[serde(rename = "_fixedContactDetail")]
    ContactDetail(FieldExtension),
    /// Variant accepting the DataRequirement extension.
    #[serde(rename = "_fixedDataRequirement")]
    DataRequirement(FieldExtension),
    /// Variant accepting the Expression extension.
    #[serde(rename = "_fixedExpression")]
    Expression(FieldExtension),
    /// Variant accepting the ParameterDefinition extension.
    #[serde(rename = "_fixedParameterDefinition")]
    ParameterDefinition(FieldExtension),
    /// Variant accepting the RelatedArtifact extension.
    #[serde(rename = "_fixedRelatedArtifact")]
    RelatedArtifact(FieldExtension),
    /// Variant accepting the TriggerDefinition extension.
    #[serde(rename = "_fixedTriggerDefinition")]
    TriggerDefinition(FieldExtension),
    /// Variant accepting the UsageContext extension.
    #[serde(rename = "_fixedUsageContext")]
    UsageContext(FieldExtension),
    /// Variant accepting the Availability extension.
    #[serde(rename = "_fixedAvailability")]
    Availability(FieldExtension),
    /// Variant accepting the ExtendedContactDetail extension.
    #[serde(rename = "_fixedExtendedContactDetail")]
    ExtendedContactDetail(FieldExtension),
    /// Variant accepting the Dosage extension.
    #[serde(rename = "_fixedDosage")]
    Dosage(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_fixedMeta")]
    Meta(FieldExtension),
}
/// Choice of types for the pattern[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionPattern {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "patternBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "patternBoolean")]
    Boolean(bool),
    /// Variant accepting the Canonical type.
    #[serde(rename = "patternCanonical")]
    Canonical(String),
    /// Variant accepting the Code type.
    #[serde(rename = "patternCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "patternDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "patternDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "patternDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "patternId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "patternInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "patternInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "patternInteger64")]
    Integer64(Integer64),
    /// Variant accepting the Markdown type.
    #[serde(rename = "patternMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "patternOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "patternPositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "patternString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "patternTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "patternUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "patternUri")]
    Uri(String),
    /// Variant accepting the Url type.
    #[serde(rename = "patternUrl")]
    Url(String),
    /// Variant accepting the Uuid type.
    #[serde(rename = "patternUuid")]
    Uuid(String),
    /// Variant accepting the Address type.
    #[serde(rename = "patternAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "patternAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "patternAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "patternAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "patternCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the CodeableReference type.
    #[serde(rename = "patternCodeableReference")]
    CodeableReference(CodeableReference),
    /// Variant accepting the Coding type.
    #[serde(rename = "patternCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "patternContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "patternCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "patternDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "patternDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "patternHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "patternIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "patternMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "patternPeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "patternQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "patternRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "patternRatio")]
    Ratio(Ratio),
    /// Variant accepting the RatioRange type.
    #[serde(rename = "patternRatioRange")]
    RatioRange(RatioRange),
    /// Variant accepting the Reference type.
    #[serde(rename = "patternReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "patternSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "patternSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "patternTiming")]
    Timing(Timing),
    /// Variant accepting the ContactDetail type.
    #[serde(rename = "patternContactDetail")]
    ContactDetail(ContactDetail),
    /// Variant accepting the DataRequirement type.
    #[serde(rename = "patternDataRequirement")]
    DataRequirement(DataRequirement),
    /// Variant accepting the Expression type.
    #[serde(rename = "patternExpression")]
    Expression(Expression),
    /// Variant accepting the ParameterDefinition type.
    #[serde(rename = "patternParameterDefinition")]
    ParameterDefinition(ParameterDefinition),
    /// Variant accepting the RelatedArtifact type.
    #[serde(rename = "patternRelatedArtifact")]
    RelatedArtifact(RelatedArtifact),
    /// Variant accepting the TriggerDefinition type.
    #[serde(rename = "patternTriggerDefinition")]
    TriggerDefinition(TriggerDefinition),
    /// Variant accepting the UsageContext type.
    #[serde(rename = "patternUsageContext")]
    UsageContext(UsageContext),
    /// Variant accepting the Availability type.
    #[serde(rename = "patternAvailability")]
    Availability(Availability),
    /// Variant accepting the ExtendedContactDetail type.
    #[serde(rename = "patternExtendedContactDetail")]
    ExtendedContactDetail(ExtendedContactDetail),
    /// Variant accepting the Dosage type.
    #[serde(rename = "patternDosage")]
    Dosage(Dosage),
    /// Variant accepting the Meta type.
    #[serde(rename = "patternMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionPattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionPatternExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_patternBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_patternBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Canonical extension.
    #[serde(rename = "_patternCanonical")]
    Canonical(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_patternCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_patternDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_patternDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_patternDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_patternId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_patternInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_patternInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_patternInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_patternMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_patternOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_patternPositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_patternString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_patternTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_patternUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_patternUri")]
    Uri(FieldExtension),
    /// Variant accepting the Url extension.
    #[serde(rename = "_patternUrl")]
    Url(FieldExtension),
    /// Variant accepting the Uuid extension.
    #[serde(rename = "_patternUuid")]
    Uuid(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_patternAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_patternAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_patternAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_patternAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_patternCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the CodeableReference extension.
    #[serde(rename = "_patternCodeableReference")]
    CodeableReference(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_patternCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_patternContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_patternCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_patternDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_patternDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_patternHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_patternIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_patternMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_patternPeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_patternQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_patternRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_patternRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the RatioRange extension.
    #[serde(rename = "_patternRatioRange")]
    RatioRange(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_patternReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_patternSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_patternSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_patternTiming")]
    Timing(FieldExtension),
    /// Variant accepting the ContactDetail extension.
    #[serde(rename = "_patternContactDetail")]
    ContactDetail(FieldExtension),
    /// Variant accepting the DataRequirement extension.
    #[serde(rename = "_patternDataRequirement")]
    DataRequirement(FieldExtension),
    /// Variant accepting the Expression extension.
    #[serde(rename = "_patternExpression")]
    Expression(FieldExtension),
    /// Variant accepting the ParameterDefinition extension.
    #[serde(rename = "_patternParameterDefinition")]
    ParameterDefinition(FieldExtension),
    /// Variant accepting the RelatedArtifact extension.
    #[serde(rename = "_patternRelatedArtifact")]
    RelatedArtifact(FieldExtension),
    /// Variant accepting the TriggerDefinition extension.
    #[serde(rename = "_patternTriggerDefinition")]
    TriggerDefinition(FieldExtension),
    /// Variant accepting the UsageContext extension.
    #[serde(rename = "_patternUsageContext")]
    UsageContext(FieldExtension),
    /// Variant accepting the Availability extension.
    #[serde(rename = "_patternAvailability")]
    Availability(FieldExtension),
    /// Variant accepting the ExtendedContactDetail extension.
    #[serde(rename = "_patternExtendedContactDetail")]
    ExtendedContactDetail(FieldExtension),
    /// Variant accepting the Dosage extension.
    #[serde(rename = "_patternDosage")]
    Dosage(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_patternMeta")]
    Meta(FieldExtension),
}
/// Sub-fields of the example field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionExampleBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionExample {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Describes the purpose of this example**

 Describes the purpose of this example among the set of examples.

 */
    #[serde(rename = "label")]
    pub label: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_label")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub label_ext: Option<FieldExtension>,
    /** **Value of Example (one of allowed types)**

 The actual value for the element, which must be one of the types allowed for this element.

 */
    #[serde(flatten)]
    pub value: ElementDefinitionExampleValue,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<ElementDefinitionExampleValueExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionExample {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionExampleBuilder {
        ElementDefinitionExampleBuilder::default()
    }
}
/// Choice of types for the value[x] field in ElementDefinitionExample
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionExampleValue {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "valueBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "valueBoolean")]
    Boolean(bool),
    /// Variant accepting the Canonical type.
    #[serde(rename = "valueCanonical")]
    Canonical(String),
    /// Variant accepting the Code type.
    #[serde(rename = "valueCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "valueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "valueDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "valueId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "valueInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "valueInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "valueInteger64")]
    Integer64(Integer64),
    /// Variant accepting the Markdown type.
    #[serde(rename = "valueMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "valueOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "valuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "valueString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "valueTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "valueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "valueUri")]
    Uri(String),
    /// Variant accepting the Url type.
    #[serde(rename = "valueUrl")]
    Url(String),
    /// Variant accepting the Uuid type.
    #[serde(rename = "valueUuid")]
    Uuid(String),
    /// Variant accepting the Address type.
    #[serde(rename = "valueAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "valueAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "valueAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "valueAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "valueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the CodeableReference type.
    #[serde(rename = "valueCodeableReference")]
    CodeableReference(CodeableReference),
    /// Variant accepting the Coding type.
    #[serde(rename = "valueCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "valueContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "valueCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "valueDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "valueHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "valueIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "valueMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "valueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "valueRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "valueRatio")]
    Ratio(Ratio),
    /// Variant accepting the RatioRange type.
    #[serde(rename = "valueRatioRange")]
    RatioRange(RatioRange),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "valueSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "valueSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "valueTiming")]
    Timing(Timing),
    /// Variant accepting the ContactDetail type.
    #[serde(rename = "valueContactDetail")]
    ContactDetail(ContactDetail),
    /// Variant accepting the DataRequirement type.
    #[serde(rename = "valueDataRequirement")]
    DataRequirement(DataRequirement),
    /// Variant accepting the Expression type.
    #[serde(rename = "valueExpression")]
    Expression(Expression),
    /// Variant accepting the ParameterDefinition type.
    #[serde(rename = "valueParameterDefinition")]
    ParameterDefinition(ParameterDefinition),
    /// Variant accepting the RelatedArtifact type.
    #[serde(rename = "valueRelatedArtifact")]
    RelatedArtifact(RelatedArtifact),
    /// Variant accepting the TriggerDefinition type.
    #[serde(rename = "valueTriggerDefinition")]
    TriggerDefinition(TriggerDefinition),
    /// Variant accepting the UsageContext type.
    #[serde(rename = "valueUsageContext")]
    UsageContext(UsageContext),
    /// Variant accepting the Availability type.
    #[serde(rename = "valueAvailability")]
    Availability(Availability),
    /// Variant accepting the ExtendedContactDetail type.
    #[serde(rename = "valueExtendedContactDetail")]
    ExtendedContactDetail(ExtendedContactDetail),
    /// Variant accepting the Dosage type.
    #[serde(rename = "valueDosage")]
    Dosage(Dosage),
    /// Variant accepting the Meta type.
    #[serde(rename = "valueMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionExampleValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionExampleValueExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_valueBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_valueBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Canonical extension.
    #[serde(rename = "_valueCanonical")]
    Canonical(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_valueCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_valueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_valueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_valueId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_valueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_valueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_valueInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_valueMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_valueOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_valuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_valueString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_valueTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_valueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_valueUri")]
    Uri(FieldExtension),
    /// Variant accepting the Url extension.
    #[serde(rename = "_valueUrl")]
    Url(FieldExtension),
    /// Variant accepting the Uuid extension.
    #[serde(rename = "_valueUuid")]
    Uuid(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_valueAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_valueAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_valueAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_valueAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_valueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the CodeableReference extension.
    #[serde(rename = "_valueCodeableReference")]
    CodeableReference(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_valueCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_valueContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_valueCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_valueDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_valueHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_valueIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_valueMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_valueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_valueRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_valueRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the RatioRange extension.
    #[serde(rename = "_valueRatioRange")]
    RatioRange(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_valueSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_valueSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_valueTiming")]
    Timing(FieldExtension),
    /// Variant accepting the ContactDetail extension.
    #[serde(rename = "_valueContactDetail")]
    ContactDetail(FieldExtension),
    /// Variant accepting the DataRequirement extension.
    #[serde(rename = "_valueDataRequirement")]
    DataRequirement(FieldExtension),
    /// Variant accepting the Expression extension.
    #[serde(rename = "_valueExpression")]
    Expression(FieldExtension),
    /// Variant accepting the ParameterDefinition extension.
    #[serde(rename = "_valueParameterDefinition")]
    ParameterDefinition(FieldExtension),
    /// Variant accepting the RelatedArtifact extension.
    #[serde(rename = "_valueRelatedArtifact")]
    RelatedArtifact(FieldExtension),
    /// Variant accepting the TriggerDefinition extension.
    #[serde(rename = "_valueTriggerDefinition")]
    TriggerDefinition(FieldExtension),
    /// Variant accepting the UsageContext extension.
    #[serde(rename = "_valueUsageContext")]
    UsageContext(FieldExtension),
    /// Variant accepting the Availability extension.
    #[serde(rename = "_valueAvailability")]
    Availability(FieldExtension),
    /// Variant accepting the ExtendedContactDetail extension.
    #[serde(rename = "_valueExtendedContactDetail")]
    ExtendedContactDetail(FieldExtension),
    /// Variant accepting the Dosage extension.
    #[serde(rename = "_valueDosage")]
    Dosage(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_valueMeta")]
    Meta(FieldExtension),
}
/// Choice of types for the minValue[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMinValue {
    /// Variant accepting the Date type.
    #[serde(rename = "minValueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "minValueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Instant type.
    #[serde(rename = "minValueInstant")]
    Instant(Instant),
    /// Variant accepting the Time type.
    #[serde(rename = "minValueTime")]
    Time(Time),
    /// Variant accepting the Decimal type.
    #[serde(rename = "minValueDecimal")]
    Decimal(f64),
    /// Variant accepting the Integer type.
    #[serde(rename = "minValueInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "minValueInteger64")]
    Integer64(Integer64),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "minValuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "minValueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Quantity type.
    #[serde(rename = "minValueQuantity")]
    Quantity(Quantity),
}
/// Extension value for ElementDefinitionMinValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMinValueExtension {
    /// Variant accepting the Date extension.
    #[serde(rename = "_minValueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_minValueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_minValueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_minValueTime")]
    Time(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_minValueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_minValueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_minValueInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_minValuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_minValueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_minValueQuantity")]
    Quantity(FieldExtension),
}
/// Choice of types for the maxValue[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMaxValue {
    /// Variant accepting the Date type.
    #[serde(rename = "maxValueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "maxValueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Instant type.
    #[serde(rename = "maxValueInstant")]
    Instant(Instant),
    /// Variant accepting the Time type.
    #[serde(rename = "maxValueTime")]
    Time(Time),
    /// Variant accepting the Decimal type.
    #[serde(rename = "maxValueDecimal")]
    Decimal(f64),
    /// Variant accepting the Integer type.
    #[serde(rename = "maxValueInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "maxValueInteger64")]
    Integer64(Integer64),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "maxValuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "maxValueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Quantity type.
    #[serde(rename = "maxValueQuantity")]
    Quantity(Quantity),
}
/// Extension value for ElementDefinitionMaxValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMaxValueExtension {
    /// Variant accepting the Date extension.
    #[serde(rename = "_maxValueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_maxValueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_maxValueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_maxValueTime")]
    Time(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_maxValueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_maxValueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_maxValueInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_maxValuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_maxValueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_maxValueQuantity")]
    Quantity(FieldExtension),
}
/// Sub-fields of the constraint field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionConstraintBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionConstraint {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Target of 'condition' reference above**

 Allows identification of which elements have their cardinalities impacted by the constraint.  Will not be referenced for constraints that do not affect cardinality.

 */
    #[serde(rename = "key")]
    pub key: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_key")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub key_ext: Option<FieldExtension>,
    /** **Why this constraint is necessary or appropriate**

 Description of why this constraint is necessary or appropriate.

 To be used if the reason for the constraint might not be intuitive to all implementers. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "requirements")]
    pub requirements: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_requirements")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub requirements_ext: Option<FieldExtension>,
    /** **[ConstraintSeverity](http://hl7.org/fhir/ValueSet/constraint-severity); error | warning**

 Identifies the impact constraint violation has on the conformance of the instance.

 This allows constraints to be asserted as "shall" (error) and "should" (warning). */
    #[serde(rename = "severity")]
    pub severity: codes::ConstraintSeverity,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_severity")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub severity_ext: Option<FieldExtension>,
    /** **Suppress warning or hint in profile**

 If true, indicates that the warning or best practice guideline should be suppressed.

 This element should only be present in a derived profile where a warning or hint has been determined to be spurious/incorrect. E.g. a warning encouraging the inclusion of an element that the profile explicitly prohibits. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "suppress")]
    pub suppress: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_suppress")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub suppress_ext: Option<FieldExtension>,
    /** **Human description of constraint**

 Text that can be used to describe the constraint in messages identifying that the constraint has been violated.

 Should be expressed in business terms as much as possible. */
    #[serde(rename = "human")]
    pub human: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_human")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub human_ext: Option<FieldExtension>,
    /** **FHIRPath expression of constraint**

 A [FHIRPath](fhirpath.html) expression of constraint that can be executed to see if this constraint is met.

 In the absense of an expression, the expression is likely not enforceable by validators, and might be missed by many systems. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_expression")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub expression_ext: Option<FieldExtension>,
    /** **Reference to original source of constraint**

 A reference to the original source of the constraint, for traceability purposes.

 This is used when, e.g. rendering, where it is not useful to present inherited constraints when rendering the snapshot. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_source")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub source_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionConstraint {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionConstraintBuilder {
        ElementDefinitionConstraintBuilder::default()
    }
}
/// Sub-fields of the binding field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBindingBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionBinding {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[BindingStrength](http://hl7.org/fhir/ValueSet/binding-strength); required | extensible | preferred | example**

 Indicates the degree of conformance expectations associated with this binding - that is, the degree to which the provided value set must be adhered to in the instances.

 For further discussion, see [Using Terminologies](terminologies.html). */
    #[serde(rename = "strength")]
    pub strength: codes::BindingStrength,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_strength")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub strength_ext: Option<FieldExtension>,
    /** **Intended use of codes in the bound value set**

 Describes the intended use of this particular set of codes.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_description")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub description_ext: Option<FieldExtension>,
    /** **Source of value set**

 Refers to the value set that identifies the set of codes the binding refers to.

 The reference may be version-specific or not (e.g. have a |[version] at the end of the canonical URL). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_valueSet")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_set_ext: Option<FieldExtension>,
    /** **Additional Bindings - more rules about the binding**

 Additional bindings that help applications implementing this element. Additional bindings do not replace the main binding but provide more information and/or context.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "additional")]
    pub additional: Vec<Option<ElementDefinitionBindingAdditional>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_additional")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub additional_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBinding {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionBindingBuilder {
        ElementDefinitionBindingBuilder::default()
    }
}
/// Sub-fields of the additional field in ElementDefinitionBinding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBindingAdditionalBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionBindingAdditional {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[AdditionalBindingPurpose](http://hl7.org/fhir/ValueSet/additional-binding-purpose); maximum | minimum | required | extensible | candidate | current | preferred | ui | starter | component**

 The use of this additional binding.

 Conformance bindings are in addition to the base binding, not instead of it. */
    #[serde(rename = "purpose")]
    pub purpose: codes::AdditionalBindingPurposeCodes,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_purpose")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub purpose_ext: Option<FieldExtension>,
    /** **The value set for the additional binding**

 The valueSet that is being bound for the purpose.

 */
    #[serde(rename = "valueSet")]
    pub value_set: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_valueSet")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_set_ext: Option<FieldExtension>,
    /** **Documentation of the purpose of use of the binding**

 Documentation of the purpose of use of the bindingproviding additional information about how it is intended to be used.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "documentation")]
    pub documentation: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_documentation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub documentation_ext: Option<FieldExtension>,
    /** **Concise documentation - for summary tables**

 Concise documentation - for summary tables.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "shortDoco")]
    pub short_doco: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_shortDoco")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub short_doco_ext: Option<FieldExtension>,
    /** **Qualifies the usage - jurisdiction, gender, workflow status etc.**

 Qualifies the usage of the binding. Typically bindings are qualified by jurisdiction, but they may also be qualified by gender, workflow status, clinical domain etc. The information to decide whether a usege context applies is usually outside the resource, determined by context, and this might present challenges for validation tooling.

 This specification does not define exactly how the context is determined for the usage; typically, this will be described in implementation guides. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "usage")]
    pub usage: Vec<Option<UsageContext>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_usage")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub usage_ext: Vec<Option<FieldExtension>>,
    /** **Whether binding can applies to all repeats, or just one**

 Whether the binding applies to all repeats, or just to any one of them. This is only relevant for elements that can repeat.

 A common pattern is to make a conformance binding to all repeats, and then make a binding that one of the repeats must conform to. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "any")]
    pub any: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_any")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub any_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBindingAdditional {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionBindingAdditionalBuilder {
        ElementDefinitionBindingAdditionalBuilder::default()
    }
}
/// Sub-fields of the mapping field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionMappingBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionMapping {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Reference to mapping declaration**

 An internal reference to the definition of a mapping.

 */
    #[serde(rename = "identity")]
    pub identity: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_identity")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub identity_ext: Option<FieldExtension>,
    /** **[MimeType](http://hl7.org/fhir/ValueSet/mimetypes); Computable language of mapping**

 Identifies the computable language in which mapping.map is expressed.

 If omitted, then there can be no expectation of computational interpretation of the mapping. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_language")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub language_ext: Option<FieldExtension>,
    /** **Details of the mapping**

 Expresses what part of the target specification corresponds to this element.

 For most mappings, the syntax is undefined.  Syntax will be provided for mappings to the RIM.  Multiple mappings may be possible and may include constraints on other resource elements that identify when a particular mapping applies. */
    #[serde(rename = "map")]
    pub map: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_map")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub map_ext: Option<FieldExtension>,
    /** **Comments about the mapping or its use**

 Comments that provide information about the mapping or its use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comment")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comment_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionMapping {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionMappingBuilder {
        ElementDefinitionMappingBuilder::default()
    }
}
/** Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used.

 **[Expression](http://hl7.org/fhir/StructureDefinition/Expression) v5.0.0**

 An expression that can be used to generate a value

 A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Expression(pub Box<ExpressionInner>);
/** Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used.

 **[Expression](http://hl7.org/fhir/StructureDefinition/Expression) v5.0.0**

 An expression that can be used to generate a value

 A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and how the result of the expression is used.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ExpressionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ExpressionInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Natural language description of the condition**

 A brief, natural language description of the condition that effectively communicates the intended semantics.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_description")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub description_ext: Option<FieldExtension>,
    /** **Short name assigned to expression for reuse**

 A short name assigned to the expression to allow for multiple reuse of the expression in the context where it is defined.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **[ExpressionLanguage](http://hl7.org/fhir/ValueSet/expression-language); text/cql | text/fhirpath | application/x-fhir-query | etc.**

 The media type of the language for the expression.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_language")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub language_ext: Option<FieldExtension>,
    /** **Expression in specified language**

 An expression in the specified language that returns a value.

 If Expression.expression and Expression.reference are both present, the Expression.expression might just be a name pointing something within the referenced content. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_expression")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub expression_ext: Option<FieldExtension>,
    /** **Where the expression is found**

 A URI that defines where the expression is found.

 If both a reference and an expression is found, the reference SHALL point to the same expression. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "reference")]
    pub reference: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_reference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub reference_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ExpressionBuilder {
    #[doc = concat!("Finalize building ", "Expression", ".")]
    pub fn build(self) -> Result<Expression, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ExpressionInner> for Expression {
    fn from(inner: ExpressionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Expression {
    type Target = ExpressionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Expression {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ExpressionBuilder {
        ExpressionBuilder::default()
    }
}
/** ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.

 **[ExtendedContactDetail](http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail) v5.0.0**

 Contact information

 Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.

 This datatype may be sparsely populated, i.e. only contain a purpose and phone number or address, but other cases could be completed filled out. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExtendedContactDetail(pub Box<ExtendedContactDetailInner>);
/** ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.

 **[ExtendedContactDetail](http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail) v5.0.0**

 Contact information

 Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.

 This datatype may be sparsely populated, i.e. only contain a purpose and phone number or address, but other cases could be completed filled out. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ExtendedContactDetailBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ExtendedContactDetailInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[Purpose](http://terminology.hl7.org/ValueSet/contactentity-type); The type of contact**

 The purpose/type of contact.

 If no purpose is defined, then these contact details may be used for any purpose. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "purpose")]
    pub purpose: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_purpose")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub purpose_ext: Option<FieldExtension>,
    /** **Name of an individual to contact**

 The name of an individual to contact, some types of contact detail are usually blank.

 If there is no named individual, the telecom/address information is not generally monitored by a specific individual. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Vec<Option<HumanName>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub name_ext: Vec<Option<FieldExtension>>,
    /** **Contact details (e.g.phone/fax/url)**

 The contact details application for the purpose defined.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "telecom")]
    pub telecom: Vec<Option<ContactPoint>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_telecom")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub telecom_ext: Vec<Option<FieldExtension>>,
    /** **Address for the contact**

 Address for the contact.

 More than 1 address would be for different purposes, and thus should be entered as a different entry,. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "address")]
    pub address: Option<Address>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_address")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub address_ext: Option<FieldExtension>,
    /** **This contact detail is handled/monitored by a specific organization**

 This contact detail is handled/monitored by a specific organization. If the name is provided in the contact, then it is referring to the named individual within this organization.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "organization")]
    pub organization: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_organization")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub organization_ext: Option<FieldExtension>,
    /** **Period that this contact was valid for usage**

 Period that this contact was valid for usage.

 If the details have multiple periods, then enter in a new ExtendedContact with the new period. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ExtendedContactDetailBuilder {
    #[doc = concat!("Finalize building ", "ExtendedContactDetail", ".")]
    pub fn build(self) -> Result<ExtendedContactDetail, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ExtendedContactDetailInner> for ExtendedContactDetail {
    fn from(inner: ExtendedContactDetailInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ExtendedContactDetail {
    type Target = ExtendedContactDetailInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ExtendedContactDetail {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ExtendedContactDetail {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ExtendedContactDetailBuilder {
        ExtendedContactDetailBuilder::default()
    }
}
/** Extension Type: Optional Extension Element - found in all resources.

 **[Extension](http://hl7.org/fhir/StructureDefinition/Extension) v5.0.0**

 Optional Extensions Element

 Optional Extension Element - found in all resources.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Extension(pub Box<ExtensionInner>);
/** Extension Type: Optional Extension Element - found in all resources.

 **[Extension](http://hl7.org/fhir/StructureDefinition/Extension) v5.0.0**

 Optional Extensions Element

 Optional Extension Element - found in all resources.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ExtensionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ExtensionInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **identifies the meaning of the extension**

 Source of the definition for the extension code - a logical name or a URL.

 The definition may point directly to a computable or human-readable definition of the extensibility codes, or it may be a logical URI as declared in some other specification. The definition SHALL be a URI for the Structure Definition defining the extension. */
    #[serde(rename = "url")]
    pub url: String,
    /** **Value of extension**

 Value of extension - must be one of a constrained set of the data types (see [Extensibility](extensibility.html) for a list).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value: Option<ExtensionValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<ExtensionValueExtension>,
}
#[cfg(feature = "builders")]
impl ExtensionBuilder {
    #[doc = concat!("Finalize building ", "Extension", ".")]
    pub fn build(self) -> Result<Extension, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ExtensionInner> for Extension {
    fn from(inner: ExtensionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Extension {
    type Target = ExtensionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Extension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Extension {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ExtensionBuilder {
        ExtensionBuilder::default()
    }
}
/// Choice of types for the value[x] field in Extension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValue {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "valueBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "valueBoolean")]
    Boolean(bool),
    /// Variant accepting the Canonical type.
    #[serde(rename = "valueCanonical")]
    Canonical(String),
    /// Variant accepting the Code type.
    #[serde(rename = "valueCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "valueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "valueDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "valueId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "valueInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "valueInteger")]
    Integer(i32),
    /// Variant accepting the Integer64 type.
    #[serde(rename = "valueInteger64")]
    Integer64(Integer64),
    /// Variant accepting the Markdown type.
    #[serde(rename = "valueMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "valueOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "valuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "valueString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "valueTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "valueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "valueUri")]
    Uri(String),
    /// Variant accepting the Url type.
    #[serde(rename = "valueUrl")]
    Url(String),
    /// Variant accepting the Uuid type.
    #[serde(rename = "valueUuid")]
    Uuid(String),
    /// Variant accepting the Address type.
    #[serde(rename = "valueAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "valueAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "valueAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "valueAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "valueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the CodeableReference type.
    #[serde(rename = "valueCodeableReference")]
    CodeableReference(CodeableReference),
    /// Variant accepting the Coding type.
    #[serde(rename = "valueCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "valueContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "valueCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "valueDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "valueHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "valueIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "valueMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "valueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "valueRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "valueRatio")]
    Ratio(Ratio),
    /// Variant accepting the RatioRange type.
    #[serde(rename = "valueRatioRange")]
    RatioRange(RatioRange),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "valueSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "valueSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "valueTiming")]
    Timing(Timing),
    /// Variant accepting the ContactDetail type.
    #[serde(rename = "valueContactDetail")]
    ContactDetail(ContactDetail),
    /// Variant accepting the DataRequirement type.
    #[serde(rename = "valueDataRequirement")]
    DataRequirement(DataRequirement),
    /// Variant accepting the Expression type.
    #[serde(rename = "valueExpression")]
    Expression(Expression),
    /// Variant accepting the ParameterDefinition type.
    #[serde(rename = "valueParameterDefinition")]
    ParameterDefinition(ParameterDefinition),
    /// Variant accepting the RelatedArtifact type.
    #[serde(rename = "valueRelatedArtifact")]
    RelatedArtifact(RelatedArtifact),
    /// Variant accepting the TriggerDefinition type.
    #[serde(rename = "valueTriggerDefinition")]
    TriggerDefinition(TriggerDefinition),
    /// Variant accepting the UsageContext type.
    #[serde(rename = "valueUsageContext")]
    UsageContext(UsageContext),
    /// Variant accepting the Availability type.
    #[serde(rename = "valueAvailability")]
    Availability(Availability),
    /// Variant accepting the ExtendedContactDetail type.
    #[serde(rename = "valueExtendedContactDetail")]
    ExtendedContactDetail(ExtendedContactDetail),
    /// Variant accepting the Dosage type.
    #[serde(rename = "valueDosage")]
    Dosage(Dosage),
    /// Variant accepting the Meta type.
    #[serde(rename = "valueMeta")]
    Meta(Meta),
}
/// Extension value for ExtensionValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValueExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_valueBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_valueBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Canonical extension.
    #[serde(rename = "_valueCanonical")]
    Canonical(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_valueCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_valueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_valueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_valueId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_valueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_valueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Integer64 extension.
    #[serde(rename = "_valueInteger64")]
    Integer64(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_valueMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_valueOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_valuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_valueString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_valueTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_valueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_valueUri")]
    Uri(FieldExtension),
    /// Variant accepting the Url extension.
    #[serde(rename = "_valueUrl")]
    Url(FieldExtension),
    /// Variant accepting the Uuid extension.
    #[serde(rename = "_valueUuid")]
    Uuid(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_valueAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_valueAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_valueAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_valueAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_valueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the CodeableReference extension.
    #[serde(rename = "_valueCodeableReference")]
    CodeableReference(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_valueCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_valueContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_valueCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_valueDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_valueHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_valueIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_valueMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_valueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_valueRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_valueRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the RatioRange extension.
    #[serde(rename = "_valueRatioRange")]
    RatioRange(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_valueSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_valueSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_valueTiming")]
    Timing(FieldExtension),
    /// Variant accepting the ContactDetail extension.
    #[serde(rename = "_valueContactDetail")]
    ContactDetail(FieldExtension),
    /// Variant accepting the DataRequirement extension.
    #[serde(rename = "_valueDataRequirement")]
    DataRequirement(FieldExtension),
    /// Variant accepting the Expression extension.
    #[serde(rename = "_valueExpression")]
    Expression(FieldExtension),
    /// Variant accepting the ParameterDefinition extension.
    #[serde(rename = "_valueParameterDefinition")]
    ParameterDefinition(FieldExtension),
    /// Variant accepting the RelatedArtifact extension.
    #[serde(rename = "_valueRelatedArtifact")]
    RelatedArtifact(FieldExtension),
    /// Variant accepting the TriggerDefinition extension.
    #[serde(rename = "_valueTriggerDefinition")]
    TriggerDefinition(FieldExtension),
    /// Variant accepting the UsageContext extension.
    #[serde(rename = "_valueUsageContext")]
    UsageContext(FieldExtension),
    /// Variant accepting the Availability extension.
    #[serde(rename = "_valueAvailability")]
    Availability(FieldExtension),
    /// Variant accepting the ExtendedContactDetail extension.
    #[serde(rename = "_valueExtendedContactDetail")]
    ExtendedContactDetail(FieldExtension),
    /// Variant accepting the Dosage extension.
    #[serde(rename = "_valueDosage")]
    Dosage(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_valueMeta")]
    Meta(FieldExtension),
}
/** HumanName Type: A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.

 **[HumanName](http://hl7.org/fhir/StructureDefinition/HumanName) v5.0.0**

 Name of a human or other living entity - parts and usage

 A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.

 Names may be changed, or repudiated, or people may have different names in different contexts. Names may be divided into parts of different type that have variable significance depending on context, though the division into parts does not always matter. With personal names, the different parts might or might not be imbued with some implicit meaning; various cultures associate different importance with the name parts and the degree to which systems must care about name parts around the world varies widely. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HumanName(pub Box<HumanNameInner>);
/** HumanName Type: A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.

 **[HumanName](http://hl7.org/fhir/StructureDefinition/HumanName) v5.0.0**

 Name of a human or other living entity - parts and usage

 A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.

 Names may be changed, or repudiated, or people may have different names in different contexts. Names may be divided into parts of different type that have variable significance depending on context, though the division into parts does not always matter. With personal names, the different parts might or might not be imbued with some implicit meaning; various cultures associate different importance with the name parts and the degree to which systems must care about name parts around the world varies widely. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "HumanNameBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct HumanNameInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[NameUse](http://hl7.org/fhir/ValueSet/name-use); usual | official | temp | nickname | anonymous | old | maiden**

 Identifies the purpose for this name.

 Applications can assume that a name is current unless it explicitly says that it is temporary or old. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::NameUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **Text representation of the full name**

 Specifies the entire name as it should be displayed e.g. on an application UI. This may be provided instead of or as well as the specific parts.

 Can provide both a text representation and parts. Applications updating a name SHALL ensure that when both text and parts are present,  no content is included in the text that isn't found in a part. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
    /** **Family name (often called 'Surname')**

 The part of a name that links to the genealogy. In some cultures (e.g. Eritrea) the family name of a son is the first name of his father.

 Family Name may be decomposed into specific parts using extensions (de, nl, es related cultures). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "family")]
    pub family: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_family")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub family_ext: Option<FieldExtension>,
    /** **Given names (not always 'first'). Includes middle names**

 Given name.

 If only initials are recorded, they may be used in place of the full name parts. Initials may be separated into multiple given names but often aren't due to paractical limitations.  This element is not called "first name" since given names do not always come first. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "given")]
    pub given: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_given")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub given_ext: Vec<Option<FieldExtension>>,
    /** **Parts that come before the name**

 Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the start of the name.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "prefix")]
    pub prefix: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_prefix")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub prefix_ext: Vec<Option<FieldExtension>>,
    /** **Parts that come after the name**

 Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the end of the name.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "suffix")]
    pub suffix: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_suffix")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub suffix_ext: Vec<Option<FieldExtension>>,
    /** **Time period when name was/is in use**

 Indicates the period of time when this name was valid for the named person.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl HumanNameBuilder {
    #[doc = concat!("Finalize building ", "HumanName", ".")]
    pub fn build(self) -> Result<HumanName, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<HumanNameInner> for HumanName {
    fn from(inner: HumanNameInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for HumanName {
    type Target = HumanNameInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for HumanName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl HumanName {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> HumanNameBuilder {
        HumanNameBuilder::default()
    }
}
/** Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.

 **[Identifier](http://hl7.org/fhir/StructureDefinition/Identifier) v5.0.0**

 An identifier intended for computation

 An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.

 The rules of the identifier.type  determine if a check digit is part of the ID value or sent separately, such as through the checkDigit extension. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identifier(pub Box<IdentifierInner>);
/** Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.

 **[Identifier](http://hl7.org/fhir/StructureDefinition/Identifier) v5.0.0**

 An identifier intended for computation

 An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.

 The rules of the identifier.type  determine if a check digit is part of the ID value or sent separately, such as through the checkDigit extension. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "IdentifierBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct IdentifierInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[IdentifierUse](http://hl7.org/fhir/ValueSet/identifier-use); usual | official | temp | secondary | old (If known)**

 The purpose of this identifier.

 Applications can assume that an identifier is permanent unless it explicitly says that it is temporary. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::IdentifierUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **[IdentifierType](http://hl7.org/fhir/ValueSet/identifier-type); Description of identifier**

 A coded type for the identifier that can be used to determine which identifier to use for a specific purpose.

 This element deals only with general categories of identifiers.  It SHOULD not be used for codes that correspond 1..1 with the Identifier.system. Some identifiers may fall into multiple categories due to common usage.   Where the system is known, a type is unnecessary because the type is always part of the system definition. However systems often need to handle identifiers where the system is not known. There is not a 1:1 relationship between type and system, since many different systems have the same type. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **The namespace for the identifier value**

 Establishes the namespace for the value - that is, an absolute URL that describes a set values that are unique.

 Identifier.system is always case sensitive. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **The value that is unique**

 The portion of the identifier typically relevant to the user and which is unique within the context of the system.

 If the value is a full URI, then the system SHALL be urn:ietf:rfc:3986.  The value's primary purpose is computational mapping.  As a result, it may be normalized for comparison purposes (e.g. removing non-significant whitespace, dashes, etc.)  A value formatted for human display can be conveyed using the [http://hl7.org/fhir/StructureDefinition/rendered-value](http://hl7.org/fhir/extensions/StructureDefinition-rendered-value.html)). Identifier.value is to be treated as case sensitive unless knowledge of the Identifier.system allows the processer to be confident that non-case-sensitive processing is safe. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **Time period when id is/was valid for use**

 Time period during which identifier is/was valid for use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
    /** **Organization that issued id (may be just text)**

 Organization that issued/manages the identifier.

 The Identifier.assigner may omit the .reference element and only contain a .display element reflecting the name or other textual information about the assigning organization. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "assigner")]
    pub assigner: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_assigner")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub assigner_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl IdentifierBuilder {
    #[doc = concat!("Finalize building ", "Identifier", ".")]
    pub fn build(self) -> Result<Identifier, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<IdentifierInner> for Identifier {
    fn from(inner: IdentifierInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Identifier {
    type Target = IdentifierInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Identifier {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> IdentifierBuilder {
        IdentifierBuilder::default()
    }
}
/** MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.

 **[MarketingStatus](http://hl7.org/fhir/StructureDefinition/MarketingStatus) v5.0.0**

 The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available

 The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MarketingStatus(pub Box<MarketingStatusInner>);
/** MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.

 **[MarketingStatus](http://hl7.org/fhir/StructureDefinition/MarketingStatus) v5.0.0**

 The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available

 The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MarketingStatusBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MarketingStatusInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Extensions that cannot be ignored even if unrecognized**

 May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.

Modifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself).

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Vec<Extension>,
    /** **The country in which the marketing authorization has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements**

 The country in which the marketing authorization has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "country")]
    pub country: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_country")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub country_ext: Option<FieldExtension>,
    /** **Where a Medicines Regulatory Agency has granted a marketing authorization for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlled terminology The controlled term and the controlled term identifier shall be specified**

 Where a Medicines Regulatory Agency has granted a marketing authorization for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlled terminology The controlled term and the controlled term identifier shall be specified.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "jurisdiction")]
    pub jurisdiction: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_jurisdiction")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub jurisdiction_ext: Option<FieldExtension>,
    /** **This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples**

 This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples.

 */
    #[serde(rename = "status")]
    pub status: CodeableConcept,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_status")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub status_ext: Option<FieldExtension>,
    /** **The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain**

 The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "dateRange")]
    pub date_range: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_dateRange")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub date_range_ext: Option<FieldExtension>,
    /** **The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain**

 The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "restoreDate")]
    pub restore_date: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_restoreDate")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub restore_date_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl MarketingStatusBuilder {
    #[doc = concat!("Finalize building ", "MarketingStatus", ".")]
    pub fn build(self) -> Result<MarketingStatus, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MarketingStatusInner> for MarketingStatus {
    fn from(inner: MarketingStatusInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for MarketingStatus {
    type Target = MarketingStatusInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for MarketingStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl MarketingStatus {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MarketingStatusBuilder {
        MarketingStatusBuilder::default()
    }
}
/** Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.

 **[Meta](http://hl7.org/fhir/StructureDefinition/Meta) v5.0.0**

 Metadata about a resource

 The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Meta(pub Box<MetaInner>);
/** Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.

 **[Meta](http://hl7.org/fhir/StructureDefinition/Meta) v5.0.0**

 Metadata about a resource

 The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MetaBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MetaInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Version specific identifier**

 The version specific identifier, as it appears in the version portion of the URL. This value changes when the resource is created, updated, or deleted.

 The server assigns this value, and ignores what the client specifies, except in the case that the server is imposing version integrity on updates/deletes. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_versionId")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub version_id_ext: Option<FieldExtension>,
    /** **When the resource version last changed**

 When the resource last changed - e.g. when the version changed.

 This element is generally omitted in instances submitted in a PUT or POST. Instead, it is populated in the response instance and when retrieving information using a GET. The server / resource manager sets this value; what a client provides is irrelevant. This is equivalent to the HTTP Last-Modified and SHOULD have the same value on a [read](http.html#read) interaction. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<Instant>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_lastUpdated")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub last_updated_ext: Option<FieldExtension>,
    /** **Identifies where the resource comes from**

 A uri that identifies the source system of the resource. This provides a minimal amount of [Provenance](provenance.html#) information that can be used to track or differentiate the source of information in the resource. The source may identify another FHIR server, document, message, database, etc.

 The exact use of the source (and the possible implied Provenance.entity.role and agent.role) is left to implementer discretion. Only one nominated source is allowed; for additional provenance details, a full Provenance resource should be used. The source may correspond to Provenance.entity.what[x] or Provenance.agent.who[x], though it may be a more general or abstract reference.

This element can be used to indicate where the current master source of a resource that has a canonical URL if the resource is no longer hosted at the canonical URL. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_source")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub source_ext: Option<FieldExtension>,
    /** **Profiles this resource claims to conform to**

 A list of profiles (references to [StructureDefinition](structuredefinition.html#) resources) that this resource claims to conform to. The URL is a reference to [StructureDefinition.url](structuredefinition-definitions.html#StructureDefinition.url).

 It is up to the server and/or other infrastructure of policy to determine whether/how these claims are verified and/or updated over time.  The list of profile URLs is a set. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub profile_ext: Vec<Option<FieldExtension>>,
    /** **[SecurityLabels](http://hl7.org/fhir/ValueSet/security-labels); Security Labels applied to this resource**

 Security labels applied to this resource. These tags connect specific resources to the overall security policy and infrastructure.

 The security labels can be updated without changing the stated version of the resource. The list of security labels is a set. Uniqueness is based the system/code, and version and display are ignored. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "security")]
    pub security: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_security")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub security_ext: Vec<Option<FieldExtension>>,
    /** **[Tags](http://hl7.org/fhir/ValueSet/common-tags); Tags applied to this resource**

 Tags applied to this resource. Tags are intended to be used to identify and relate resources to process and workflow, and applications are not required to consider the tags when interpreting the meaning of a resource.

 The tags can be updated without changing the stated version of the resource. The list of tags is a set. Uniqueness is based the system/code, and version and display are ignored. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "tag")]
    pub tag: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_tag")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub tag_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl MetaBuilder {
    #[doc = concat!("Finalize building ", "Meta", ".")]
    pub fn build(self) -> Result<Meta, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MetaInner> for Meta {
    fn from(inner: MetaInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Meta {
    type Target = MetaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Meta {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MetaBuilder {
        MetaBuilder::default()
    }
}
/** MonetaryComponent Type: Availability data for an {item}.

 **[MonetaryComponent](http://hl7.org/fhir/StructureDefinition/MonetaryComponent) v5.0.0**

 Availability data for an {item}

 Availability data for an {item}.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MonetaryComponent(pub Box<MonetaryComponentInner>);
/** MonetaryComponent Type: Availability data for an {item}.

 **[MonetaryComponent](http://hl7.org/fhir/StructureDefinition/MonetaryComponent) v5.0.0**

 Availability data for an {item}

 Availability data for an {item}.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MonetaryComponentBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MonetaryComponentInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[PriceComponentType](http://hl7.org/fhir/ValueSet/price-component-type); base | surcharge | deduction | discount | tax | informational**

 base | surcharge | deduction | discount | tax | informational.

 */
    #[serde(rename = "type")]
    pub r#type: codes::PriceComponentType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **[PriceComponentCode](); Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.**

 Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Factor used for calculating this component**

 Factor used for calculating this component.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "factor")]
    pub factor: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_factor")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub factor_ext: Option<FieldExtension>,
    /** **Explicit value amount to be used**

 Explicit value amount to be used.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "amount")]
    pub amount: Option<Money>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_amount")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub amount_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl MonetaryComponentBuilder {
    #[doc = concat!("Finalize building ", "MonetaryComponent", ".")]
    pub fn build(self) -> Result<MonetaryComponent, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MonetaryComponentInner> for MonetaryComponent {
    fn from(inner: MonetaryComponentInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for MonetaryComponent {
    type Target = MonetaryComponentInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for MonetaryComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl MonetaryComponent {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MonetaryComponentBuilder {
        MonetaryComponentBuilder::default()
    }
}
/** Money Type: An amount of economic utility in some recognized currency.

 **[Money](http://hl7.org/fhir/StructureDefinition/Money) v5.0.0**

 An amount of economic utility in some recognized currency

 An amount of economic utility in some recognized currency.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money(pub Box<MoneyInner>);
/** Money Type: An amount of economic utility in some recognized currency.

 **[Money](http://hl7.org/fhir/StructureDefinition/Money) v5.0.0**

 An amount of economic utility in some recognized currency

 An amount of economic utility in some recognized currency.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MoneyBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MoneyInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 Numerical value (with implicit precision).

 Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[CurrencyCode](http://hl7.org/fhir/ValueSet/currencies); ISO 4217 Currency Code**

 ISO 4217 Currency Code.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_currency")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub currency_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl MoneyBuilder {
    #[doc = concat!("Finalize building ", "Money", ".")]
    pub fn build(self) -> Result<Money, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MoneyInner> for Money {
    fn from(inner: MoneyInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Money {
    type Target = MoneyInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Money {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Money {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MoneyBuilder {
        MoneyBuilder::default()
    }
}
/** Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.

 **[Narrative](http://hl7.org/fhir/StructureDefinition/Narrative) v5.0.0**

 Human-readable summary of the resource (essential clinical and business information)

 A human-readable summary of the resource conveying the essential clinical and business information for the resource.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Narrative(pub Box<NarrativeInner>);
/** Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.

 **[Narrative](http://hl7.org/fhir/StructureDefinition/Narrative) v5.0.0**

 Human-readable summary of the resource (essential clinical and business information)

 A human-readable summary of the resource conveying the essential clinical and business information for the resource.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "NarrativeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct NarrativeInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[NarrativeStatus](http://hl7.org/fhir/ValueSet/narrative-status); generated | extensions | additional | empty**

 The status of the narrative - whether it's entirely generated (from just the defined data or the extensions too), or whether a human authored it and it may contain additional data.

 */
    #[serde(rename = "status")]
    pub status: codes::NarrativeStatus,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_status")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub status_ext: Option<FieldExtension>,
    /** **Limited xhtml content**

 The actual narrative content, a stripped down version of XHTML.

 The contents of the html element are an XHTML fragment containing only the basic html formatting elements described in chapters 7-11 and 15 of the HTML 4.0 standard, <a> elements (either name or href), images and internally contained stylesheets. The XHTML content SHALL NOT contain a head, a body, external stylesheet references, scripts, forms, base/link/xlink, frames, iframes and objects. */
    #[serde(rename = "div")]
    pub div: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_div")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub div_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl NarrativeBuilder {
    #[doc = concat!("Finalize building ", "Narrative", ".")]
    pub fn build(self) -> Result<Narrative, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<NarrativeInner> for Narrative {
    fn from(inner: NarrativeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Narrative {
    type Target = NarrativeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Narrative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Narrative {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> NarrativeBuilder {
        NarrativeBuilder::default()
    }
}
/** ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.

 **[ParameterDefinition](http://hl7.org/fhir/StructureDefinition/ParameterDefinition) v5.0.0**

 Definition of a parameter to a module

 The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParameterDefinition(pub Box<ParameterDefinitionInner>);
/** ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.

 **[ParameterDefinition](http://hl7.org/fhir/StructureDefinition/ParameterDefinition) v5.0.0**

 Definition of a parameter to a module

 The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ParameterDefinitionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ParameterDefinitionInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Name used to access the parameter value**

 The name of the parameter used to allow access to the value of the parameter in evaluation contexts.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **[ParameterUse](http://hl7.org/fhir/ValueSet/operation-parameter-use); in | out**

 Whether the parameter is input or output for the module.

 */
    #[serde(rename = "use")]
    pub r#use: codes::OperationParameterUse,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **Minimum cardinality**

 The minimum number of times this parameter SHALL appear in the request or response.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "min")]
    pub min: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_min")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_ext: Option<FieldExtension>,
    /** **Maximum cardinality (a number of *)**

 The maximum number of times this element is permitted to appear in the request or response.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_max")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_ext: Option<FieldExtension>,
    /** **A brief description of the parameter**

 A brief discussion of what the parameter is for and how it is used by the module.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "documentation")]
    pub documentation: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_documentation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub documentation_ext: Option<FieldExtension>,
    /** **[FHIRTypes](http://hl7.org/fhir/ValueSet/fhir-types); What type of value**

 The type of the parameter.

 */
    #[serde(rename = "type")]
    pub r#type: codes::FHIRTypes,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **What profile the value is expected to be**

 If specified, this indicates a profile that the input data must conform to, or that the output data will conform to.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub profile_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ParameterDefinitionBuilder {
    #[doc = concat!("Finalize building ", "ParameterDefinition", ".")]
    pub fn build(self) -> Result<ParameterDefinition, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ParameterDefinitionInner> for ParameterDefinition {
    fn from(inner: ParameterDefinitionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ParameterDefinition {
    type Target = ParameterDefinitionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ParameterDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ParameterDefinition {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ParameterDefinitionBuilder {
        ParameterDefinitionBuilder::default()
    }
}
/** Period Type: A time period defined by a start and end date and optionally time.

 **[Period](http://hl7.org/fhir/StructureDefinition/Period) v5.0.0**

 Time range defined by start and end date/time

 A time period defined by a start and end date and optionally time.

 A Period specifies a range of time; the context of use will specify whether the entire range applies (e.g. "the patient was an inpatient of the hospital for this time range") or one value from the range applies (e.g. "give to the patient between these two times").

Period is not used for a duration (a measure of elapsed time). See [Duration](datatypes.html#Duration). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Period(pub Box<PeriodInner>);
/** Period Type: A time period defined by a start and end date and optionally time.

 **[Period](http://hl7.org/fhir/StructureDefinition/Period) v5.0.0**

 Time range defined by start and end date/time

 A time period defined by a start and end date and optionally time.

 A Period specifies a range of time; the context of use will specify whether the entire range applies (e.g. "the patient was an inpatient of the hospital for this time range") or one value from the range applies (e.g. "give to the patient between these two times").

Period is not used for a duration (a measure of elapsed time). See [Duration](datatypes.html#Duration). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "PeriodBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct PeriodInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Starting time with inclusive boundary**

 The start of the period. The boundary is inclusive.

 If the low element is missing, the meaning is that the low boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "start")]
    pub start: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_start")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub start_ext: Option<FieldExtension>,
    /** **End time with inclusive boundary, if not ongoing**

 The end of the period. If the end of the period is missing, it means no end was known or planned at the time the instance was created. The start may be in the past, and the end date in the future, which means that period is expected/planned to end at that time.

 The end value includes any matching date/time. i.e. 2012-02-03T10:00:00 is in a period that has an end value of 2012-02-03. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "end")]
    pub end: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_end")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub end_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl PeriodBuilder {
    #[doc = concat!("Finalize building ", "Period", ".")]
    pub fn build(self) -> Result<Period, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<PeriodInner> for Period {
    fn from(inner: PeriodInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Period {
    type Target = PeriodInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Period {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Period {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> PeriodBuilder {
        PeriodBuilder::default()
    }
}
/** ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.

 **[ProductShelfLife](http://hl7.org/fhir/StructureDefinition/ProductShelfLife) v5.0.0**

 The shelf-life and storage information for a medicinal product item or container can be described using this class

 The shelf-life and storage information for a medicinal product item or container can be described using this class.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProductShelfLife(pub Box<ProductShelfLifeInner>);
/** ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.

 **[ProductShelfLife](http://hl7.org/fhir/StructureDefinition/ProductShelfLife) v5.0.0**

 The shelf-life and storage information for a medicinal product item or container can be described using this class

 The shelf-life and storage information for a medicinal product item or container can be described using this class.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ProductShelfLifeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ProductShelfLifeInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Extensions that cannot be ignored even if unrecognized**

 May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.

Modifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself).

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Vec<Extension>,
    /** **This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified**

 This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used**

 The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub period: Option<ProductShelfLifePeriod>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<ProductShelfLifePeriodExtension>,
    /** **Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified**

 Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "specialPrecautionsForStorage")]
    pub special_precautions_for_storage: Vec<Option<CodeableConcept>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_specialPrecautionsForStorage")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub special_precautions_for_storage_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ProductShelfLifeBuilder {
    #[doc = concat!("Finalize building ", "ProductShelfLife", ".")]
    pub fn build(self) -> Result<ProductShelfLife, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ProductShelfLifeInner> for ProductShelfLife {
    fn from(inner: ProductShelfLifeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ProductShelfLife {
    type Target = ProductShelfLifeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ProductShelfLife {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ProductShelfLife {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ProductShelfLifeBuilder {
        ProductShelfLifeBuilder::default()
    }
}
/// Choice of types for the period[x] field in ProductShelfLife
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProductShelfLifePeriod {
    /// Variant accepting the Duration type.
    #[serde(rename = "periodDuration")]
    Duration(Duration),
    /// Variant accepting the String type.
    #[serde(rename = "periodString")]
    String(String),
}
/// Extension value for ProductShelfLifePeriod.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProductShelfLifePeriodExtension {
    /// Variant accepting the Duration extension.
    #[serde(rename = "_periodDuration")]
    Duration(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_periodString")]
    String(FieldExtension),
}
/** Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 **[Quantity](http://hl7.org/fhir/StructureDefinition/Quantity) v5.0.0**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Quantity(pub Box<QuantityInner>);
/** Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 **[Quantity](http://hl7.org/fhir/StructureDefinition/Quantity) v5.0.0**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "QuantityBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct QuantityInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comparator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comparator_ext: Option<FieldExtension>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_unit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub unit_ext: Option<FieldExtension>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl QuantityBuilder {
    #[doc = concat!("Finalize building ", "Quantity", ".")]
    pub fn build(self) -> Result<Quantity, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<QuantityInner> for Quantity {
    fn from(inner: QuantityInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Quantity {
    type Target = QuantityInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Quantity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Quantity {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> QuantityBuilder {
        QuantityBuilder::default()
    }
}
/** Range Type: A set of ordered Quantities defined by a low and high limit.

 **[Range](http://hl7.org/fhir/StructureDefinition/Range) v5.0.0**

 Set of values bounded by low and high

 A set of ordered Quantities defined by a low and high limit.

 The stated low and high value are assumed to have arbitrarily high precision when it comes to determining which values are in the range. I.e. 1.99 is not in the range 2 -> 3. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Range(pub Box<RangeInner>);
/** Range Type: A set of ordered Quantities defined by a low and high limit.

 **[Range](http://hl7.org/fhir/StructureDefinition/Range) v5.0.0**

 Set of values bounded by low and high

 A set of ordered Quantities defined by a low and high limit.

 The stated low and high value are assumed to have arbitrarily high precision when it comes to determining which values are in the range. I.e. 1.99 is not in the range 2 -> 3. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RangeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RangeInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Low limit**

 The low limit. The boundary is inclusive.

 If the low element is missing, the low boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "low")]
    pub low: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_low")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub low_ext: Option<FieldExtension>,
    /** **High limit**

 The high limit. The boundary is inclusive.

 If the high element is missing, the high boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "high")]
    pub high: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_high")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub high_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RangeBuilder {
    #[doc = concat!("Finalize building ", "Range", ".")]
    pub fn build(self) -> Result<Range, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RangeInner> for Range {
    fn from(inner: RangeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Range {
    type Target = RangeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Range {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Range {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RangeBuilder {
        RangeBuilder::default()
    }
}
/** Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.

 **[Ratio](http://hl7.org/fhir/StructureDefinition/Ratio) v5.0.0**

 A ratio of two Quantity values - a numerator and a denominator

 A relationship of two Quantity values - expressed as a numerator and a denominator.

 The Ratio datatype should only be used to express a relationship of two numbers if the relationship cannot be suitably expressed using a Quantity and a common unit.  Where the denominator value is known to be fixed to "1", Quantity should be used instead of Ratio. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ratio(pub Box<RatioInner>);
/** Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.

 **[Ratio](http://hl7.org/fhir/StructureDefinition/Ratio) v5.0.0**

 A ratio of two Quantity values - a numerator and a denominator

 A relationship of two Quantity values - expressed as a numerator and a denominator.

 The Ratio datatype should only be used to express a relationship of two numbers if the relationship cannot be suitably expressed using a Quantity and a common unit.  Where the denominator value is known to be fixed to "1", Quantity should be used instead of Ratio. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RatioBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RatioInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerator value**

 The value of the numerator.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "numerator")]
    pub numerator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_numerator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub numerator_ext: Option<FieldExtension>,
    /** **Denominator value**

 The value of the denominator.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "denominator")]
    pub denominator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_denominator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub denominator_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RatioBuilder {
    #[doc = concat!("Finalize building ", "Ratio", ".")]
    pub fn build(self) -> Result<Ratio, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RatioInner> for Ratio {
    fn from(inner: RatioInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Ratio {
    type Target = RatioInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Ratio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Ratio {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RatioBuilder {
        RatioBuilder::default()
    }
}
/** RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.

 **[RatioRange](http://hl7.org/fhir/StructureDefinition/RatioRange) v5.0.0**

 Range of ratio values

 A range of ratios expressed as a low and high numerator and a denominator.

 The stated low and high value are assumed to have arbitrarily high precision when it comes to determining which values are in the range. I.e. 1.99 is not in the range 2 -> 3. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RatioRange(pub Box<RatioRangeInner>);
/** RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.

 **[RatioRange](http://hl7.org/fhir/StructureDefinition/RatioRange) v5.0.0**

 Range of ratio values

 A range of ratios expressed as a low and high numerator and a denominator.

 The stated low and high value are assumed to have arbitrarily high precision when it comes to determining which values are in the range. I.e. 1.99 is not in the range 2 -> 3. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RatioRangeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RatioRangeInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Low Numerator limit**

 The value of the low limit numerator.

 If the low element is missing, the low boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "lowNumerator")]
    pub low_numerator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_lowNumerator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub low_numerator_ext: Option<FieldExtension>,
    /** **High Numerator limit**

 The value of the high limit numerator.

 If the high element is missing, the high boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "highNumerator")]
    pub high_numerator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_highNumerator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub high_numerator_ext: Option<FieldExtension>,
    /** **Denominator value**

 The value of the denominator.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "denominator")]
    pub denominator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_denominator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub denominator_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RatioRangeBuilder {
    #[doc = concat!("Finalize building ", "RatioRange", ".")]
    pub fn build(self) -> Result<RatioRange, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RatioRangeInner> for RatioRange {
    fn from(inner: RatioRangeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for RatioRange {
    type Target = RatioRangeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for RatioRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RatioRange {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RatioRangeBuilder {
        RatioRangeBuilder::default()
    }
}
/** Reference Type: A reference from one resource to another.

 **[Reference](http://hl7.org/fhir/StructureDefinition/Reference) v5.0.0**

 A reference from one resource to another

 A reference from one resource to another.

 References SHALL be a reference to an actual FHIR resource, and SHALL be resolvable (allowing for access control, temporary unavailability, etc.). Resolution can be either by retrieval from the URL, or, where applicable by resource type, by treating an absolute reference as a canonical URL and looking it up in a local registry/repository. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Reference(pub Box<ReferenceInner>);
/** Reference Type: A reference from one resource to another.

 **[Reference](http://hl7.org/fhir/StructureDefinition/Reference) v5.0.0**

 A reference from one resource to another

 A reference from one resource to another.

 References SHALL be a reference to an actual FHIR resource, and SHALL be resolvable (allowing for access control, temporary unavailability, etc.). Resolution can be either by retrieval from the URL, or, where applicable by resource type, by treating an absolute reference as a canonical URL and looking it up in a local registry/repository. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ReferenceBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ReferenceInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Literal reference, Relative, internal or absolute URL**

 A reference to a location at which the other resource is found. The reference may be a relative reference, in which case it is relative to the service base URL, or an absolute URL that resolves to the location where the resource is found. The reference may be version specific or not. If the reference is not to a FHIR RESTful server, then it should be assumed to be version specific. Internal fragment references (start with '#') refer to contained resources.

 Using absolute URLs provides a stable scalable approach suitable for a cloud/web context, while using relative/logical references provides a flexible approach suitable for use when trading across closed eco-system boundaries.   Absolute URLs do not need to point to a FHIR RESTful server, though this is the preferred approach. If the URL conforms to the structure "[type]/[id]" then it should be assumed that the reference is to a FHIR RESTful server. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "reference")]
    pub reference: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_reference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub reference_ext: Option<FieldExtension>,
    /** **[FHIRResourceTypeExt](http://hl7.org/fhir/ValueSet/resource-types); Type the reference refers to (e.g. "Patient") - must be a resource in resources**

 The expected type of the target of the reference. If both Reference.type and Reference.reference are populated and Reference.reference is a FHIR URL, both SHALL be consistent.

The type is the Canonical URL of Resource Definition that is the type this reference refers to. References are URLs that are relative to http://hl7.org/fhir/StructureDefinition/ e.g. "Patient" is a reference to http://hl7.org/fhir/StructureDefinition/Patient. Absolute URLs are only allowed for logical models (and can only be used in references in logical models, not resources).

 This element is used to indicate the type of  the target of the reference. This may be used which ever of the other elements are populated (or not). In some cases, the type of the target may be determined by inspection of the reference (e.g. a known RESTful URL) or by resolving the target of the reference. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Logical reference, when literal reference is not known**

 An identifier for the target resource. This is used when there is no way to reference the other resource directly, either because the entity it represents is not available through a FHIR server, or because there is no way for the author of the resource to convert a known identifier to an actual location. There is no requirement that a Reference.identifier point to something that is actually exposed as a FHIR instance, but it SHALL point to a business concept that would be expected to be exposed as a FHIR instance, and that instance would need to be of a FHIR resource type allowed by the reference.

 When an identifier is provided in place of a reference, any system processing the reference will only be able to resolve the identifier to a reference if it understands the business context in which the identifier is used. Sometimes this is global (e.g. a national identifier) but often it is not. For this reason, none of the useful mechanisms described for working with references (e.g. chaining, includes) are possible, nor should servers be expected to be able resolve the reference. Servers may accept an identifier based reference untouched, resolve it, and/or reject it - see CapabilityStatement.rest.resource.referencePolicy.

When both an identifier and a literal reference are provided, the literal reference is preferred. Applications processing the resource are allowed - but not required - to check that the identifier matches the literal reference

Applications converting a logical reference to a literal reference may choose to leave the logical reference present, or remove it.

Reference is intended to point to a structure that can potentially be expressed as a FHIR resource, though there is no need for it to exist as an actual FHIR resource instance - except in as much as an application wishes to actual find the target of the reference. The content referred to be the identifier must meet the logical constraints implied by any limitations on what resource types are permitted for the reference.  For example, it would not be legitimate to send the identifier for a drug prescription if the type were Reference(Observation|DiagnosticReport).  One of the use-cases for Reference.identifier is the situation where no FHIR representation exists (where the type is Reference (Any).

This element only allows for a single identifier. In the case where additional identifers are required, use the [http://hl7.org/fhir/StructureDefinition/additionalIdentifier](http://hl7.org/fhir/extensions/StructureDefinition-additionalIdentifier.html) extension. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "identifier")]
    pub identifier: Option<Identifier>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_identifier")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub identifier_ext: Option<FieldExtension>,
    /** **Text alternative for the resource**

 Plain text narrative that identifies the resource in addition to the resource reference.

 This is generally not the same as the Resource.text of the referenced resource.  The purpose is to identify what's being referenced, not to fully describe it. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "display")]
    pub display: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_display")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub display_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ReferenceBuilder {
    #[doc = concat!("Finalize building ", "Reference", ".")]
    pub fn build(self) -> Result<Reference, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ReferenceInner> for Reference {
    fn from(inner: ReferenceInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Reference {
    type Target = ReferenceInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Reference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Reference {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ReferenceBuilder {
        ReferenceBuilder::default()
    }
}
/** RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.

 **[RelatedArtifact](http://hl7.org/fhir/StructureDefinition/RelatedArtifact) v5.0.0**

 Related artifacts for a knowledge resource

 Related artifacts such as additional documentation, justification, or bibliographic references.

 Each related artifact is either an attachment, or a reference to another knowledge resource, but not both. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelatedArtifact(pub Box<RelatedArtifactInner>);
/** RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.

 **[RelatedArtifact](http://hl7.org/fhir/StructureDefinition/RelatedArtifact) v5.0.0**

 Related artifacts for a knowledge resource

 Related artifacts such as additional documentation, justification, or bibliographic references.

 Each related artifact is either an attachment, or a reference to another knowledge resource, but not both. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RelatedArtifactBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RelatedArtifactInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[RelatedArtifactType](http://hl7.org/fhir/ValueSet/related-artifact-type); documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as**

 The type of relationship to the related artifact.

 The presence of both sides of a relationship type (e.g. successor and predecessor) is required to support use cases where one side of a relationship is not represented in FHIR. However, this feature SHALL NOT be used to create bi-directional resource links in FHIR instances. Specifically, following the methodology of "new points to old" and "many points to one", when using related artifact elements to describe and reference FHIR resources, the type element SHALL be drawn from the fhir-related-artifact-type ValueSet. */
    #[serde(rename = "type")]
    pub r#type: codes::RelatedArtifactType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **[RelatedArtifactClassifier](http://hl7.org/fhir/ValueSet/citation-artifact-classifier); Additional classifiers**

 Provides additional classifiers of the related artifact.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "classifier")]
    pub classifier: Vec<Option<CodeableConcept>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_classifier")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub classifier_ext: Vec<Option<FieldExtension>>,
    /** **Short label**

 A short label that can be used to reference the citation from elsewhere in the containing artifact, such as a footnote index.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_label")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub label_ext: Option<FieldExtension>,
    /** **Brief description of the related artifact**

 A brief description of the document or knowledge resource being referenced, suitable for display to a consumer.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "display")]
    pub display: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_display")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub display_ext: Option<FieldExtension>,
    /** **Bibliographic citation for the artifact**

 A bibliographic citation for the related artifact. This text SHOULD be formatted according to an accepted citation format.

 Additional structured information about citations should be captured as extensions. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "citation")]
    pub citation: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_citation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub citation_ext: Option<FieldExtension>,
    /** **What document is being referenced**

 The document being referenced, represented as an attachment. This is exclusive with the resource element.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "document")]
    pub document: Option<Attachment>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_document")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub document_ext: Option<FieldExtension>,
    /** **What artifact is being referenced**

 The related artifact, such as a library, value set, profile, or other knowledge resource.

 If the type is predecessor, this is a reference to the succeeding knowledge resource. If the type is successor, this is a reference to the prior knowledge resource. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_resource")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub resource_ext: Option<FieldExtension>,
    /** **What artifact, if not a conformance resource**

 The related artifact, if the artifact is not a canonical resource, or a resource reference to a canonical resource.

 If both resource and resourceReference are present, they SHOULD be consistent and reference the same resource. Although relatedArtifact is intended to support references to definitional resources, there are cases where non-definitional resources can be definitional (such as Location where the kind is mode). Related artifacts SHOULD be used to reference definitional resources, and profiles SHOULD be used to make that explicit for particular use cases. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "resourceReference")]
    pub resource_reference: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_resourceReference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub resource_reference_ext: Option<FieldExtension>,
    /** **[RelatedArtifactPublicationStatus](http://hl7.org/fhir/ValueSet/publication-status); draft | active | retired | unknown**

 The publication status of the artifact being referred to.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "publicationStatus")]
    pub publication_status: Option<codes::PublicationStatus>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_publicationStatus")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub publication_status_ext: Option<FieldExtension>,
    /** **Date of publication of the artifact being referred to**

 The date of publication of the artifact being referred to.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<Date>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_publicationDate")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub publication_date_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RelatedArtifactBuilder {
    #[doc = concat!("Finalize building ", "RelatedArtifact", ".")]
    pub fn build(self) -> Result<RelatedArtifact, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RelatedArtifactInner> for RelatedArtifact {
    fn from(inner: RelatedArtifactInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for RelatedArtifact {
    type Target = RelatedArtifactInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for RelatedArtifact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RelatedArtifact {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RelatedArtifactBuilder {
        RelatedArtifactBuilder::default()
    }
}
/** SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.

 **[SampledData](http://hl7.org/fhir/StructureDefinition/SampledData) v5.0.0**

 A series of measurements taken by a device

 A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.

 The data is not interpretable without at least origin, period, and dimensions, but these are optional to allow a separation between the template of measurement and the actual measurement, such as between DeviceCapabilities and DeviceLog.  When providing a summary view (for example with Observation.value[x]) SampledData should be represented with a brief display text such as "Sampled Data". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SampledData(pub Box<SampledDataInner>);
/** SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.

 **[SampledData](http://hl7.org/fhir/StructureDefinition/SampledData) v5.0.0**

 A series of measurements taken by a device

 A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.

 The data is not interpretable without at least origin, period, and dimensions, but these are optional to allow a separation between the template of measurement and the actual measurement, such as between DeviceCapabilities and DeviceLog.  When providing a summary view (for example with Observation.value[x]) SampledData should be represented with a brief display text such as "Sampled Data". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "SampledDataBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct SampledDataInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Zero value and units**

 The base quantity that a measured value of zero represents. In addition, this provides the units of the entire measurement series.

 */
    #[serde(rename = "origin")]
    pub origin: Quantity,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_origin")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub origin_ext: Option<FieldExtension>,
    /** **Number of intervalUnits between samples**

 Amount of intervalUnits between samples, e.g. milliseconds for time-based sampling.

 This is usually a whole number. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_interval")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub interval_ext: Option<FieldExtension>,
    /** **[Units](http://hl7.org/fhir/ValueSet/ucum-units); The measurement unit of the interval between samples**

 The measurement unit in which the sample interval is expressed.

 */
    #[serde(rename = "intervalUnit")]
    pub interval_unit: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_intervalUnit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub interval_unit_ext: Option<FieldExtension>,
    /** **Multiply data by this before adding to origin**

 A correction factor that is applied to the sampled data points before they are added to the origin.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "factor")]
    pub factor: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_factor")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub factor_ext: Option<FieldExtension>,
    /** **Lower limit of detection**

 The lower limit of detection of the measured points. This is needed if any of the data points have the value "L" (lower than detection limit).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "lowerLimit")]
    pub lower_limit: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_lowerLimit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub lower_limit_ext: Option<FieldExtension>,
    /** **Upper limit of detection**

 The upper limit of detection of the measured points. This is needed if any of the data points have the value "U" (higher than detection limit).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "upperLimit")]
    pub upper_limit: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_upperLimit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub upper_limit_ext: Option<FieldExtension>,
    /** **Number of sample points at each time point**

 The number of sample points at each time point. If this value is greater than one, then the dimensions will be interlaced - all the sample points for a point in time will be recorded at once.

 If there is more than one dimension, the code for the type of data will define the meaning of the dimensions (typically ECG data). */
    #[serde(rename = "dimensions")]
    pub dimensions: NonZeroU32,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_dimensions")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub dimensions_ext: Option<FieldExtension>,
    /** **Defines the codes used in the data**

 Reference to ConceptMap that defines the codes used in the data.

 The ConceptMap cannot define meanings for the codes 'E', 'U', or 'L' (nor 'e', 'u', or 'l'). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "codeMap")]
    pub code_map: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_codeMap")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_map_ext: Option<FieldExtension>,
    /** **Offsets, typically in time, at which data values were taken**

 A series of data points which are decimal values separated by a single space (character u20).  The units in which the offsets are expressed are found in intervalUnit.  The absolute point at which the measurements begin SHALL be conveyed outside the scope of this datatype, e.g. Observation.effectiveDateTime for a timing offset.

 If offsets is present, the number of data points must be equal to the number of offsets mlutipled by the dimensions. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "offsets")]
    pub offsets: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_offsets")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub offsets_ext: Option<FieldExtension>,
    /** **Decimal values with spaces, or "E" | "U" | "L", or another code**

 A series of data points which are decimal values or codes separated by a single space (character u20). The special codes "E" (error), "L" (below detection limit) and "U" (above detection limit) are also defined for used in place of decimal values.

 The data may be missing if it is omitted for summarization purposes. In general, data is required for any actual use of a SampledData. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "data")]
    pub data: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_data")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub data_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl SampledDataBuilder {
    #[doc = concat!("Finalize building ", "SampledData", ".")]
    pub fn build(self) -> Result<SampledData, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<SampledDataInner> for SampledData {
    fn from(inner: SampledDataInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for SampledData {
    type Target = SampledDataInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for SampledData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl SampledData {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> SampledDataBuilder {
        SampledDataBuilder::default()
    }
}
/** Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.

 **[Signature](http://hl7.org/fhir/StructureDefinition/Signature) v5.0.0**

 A Signature - XML DigSig, JWS, Graphical image of signature, etc.

 A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.

 The elements of the Signature Resource are for ease of access of these elements. For digital signatures (Xml DigSig, JWS), the non-repudiation proof comes from the Signature  validation, which includes validation of the referenced objects (e.g. Resources) (a.k.a., Content) in the XML-Signature Detached form. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Signature(pub Box<SignatureInner>);
/** Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.

 **[Signature](http://hl7.org/fhir/StructureDefinition/Signature) v5.0.0**

 A Signature - XML DigSig, JWS, Graphical image of signature, etc.

 A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.

 The elements of the Signature Resource are for ease of access of these elements. For digital signatures (Xml DigSig, JWS), the non-repudiation proof comes from the Signature  validation, which includes validation of the referenced objects (e.g. Resources) (a.k.a., Content) in the XML-Signature Detached form. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "SignatureBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct SignatureInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[SignatureType](http://hl7.org/fhir/ValueSet/signature-type); Indication of the reason the entity signed the object(s)**

 An indication of the reason that the entity signed this document. This may be explicitly included as part of the signature information and can be used when determining accountability for various actions concerning the document.

 Examples include attesting to: authorship, correct transcription, and witness of specific event. Also known as a &quot;Commitment Type Indication&quot;. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub r#type_ext: Vec<Option<FieldExtension>>,
    /** **When the signature was created**

 When the digital signature was signed.

 This should agree with the information in the signature. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "when")]
    pub when: Option<Instant>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_when")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub when_ext: Option<FieldExtension>,
    /** **Who signed**

 A reference to an application-usable description of the identity that signed  (e.g. the signature used their private key).

 This should agree with the information in the signature. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "who")]
    pub who: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_who")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub who_ext: Option<FieldExtension>,
    /** **The party represented**

 A reference to an application-usable description of the identity that is represented by the signature.

 The party that can't sign. For example a child. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_onBehalfOf")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub on_behalf_of_ext: Option<FieldExtension>,
    /** **[MimeType](http://hl7.org/fhir/ValueSet/mimetypes); The technical format of the signed resources**

 A mime type that indicates the technical format of the target resources signed by the signature.

 "xml", "json" and "ttl" are allowed, which describe the simple encodings described in the specification (and imply appropriate bundle support). Otherwise, mime types are legal here. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "targetFormat")]
    pub target_format: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_targetFormat")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub target_format_ext: Option<FieldExtension>,
    #[doc = " **[MimeType](http://hl7.org/fhir/ValueSet/mimetypes); The technical format of the signature** \n\n A mime type that indicates the technical format of the signature. Important mime types are application/signature+xml for X ML DigSig, application/jose for JWS, and image/* for a graphical image of a signature, etc. \n\n "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sigFormat")]
    pub sig_format: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sigFormat")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub sig_format_ext: Option<FieldExtension>,
    /** **The actual signature content (XML DigSig. JWS, picture, etc.)**

 The base64 encoding of the Signature content. When signature is not recorded electronically this element would be empty.

 Where the signature type is an XML DigSig, the signed content is a FHIR Resource(s), the signature is of the XML form of the Resource(s) using  XML-Signature (XMLDIG) "Detached Signature" form. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "data")]
    pub data: Option<Base64Binary>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_data")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub data_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl SignatureBuilder {
    #[doc = concat!("Finalize building ", "Signature", ".")]
    pub fn build(self) -> Result<Signature, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<SignatureInner> for Signature {
    fn from(inner: SignatureInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Signature {
    type Target = SignatureInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Signature {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> SignatureBuilder {
        SignatureBuilder::default()
    }
}
/** Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.

 **[Timing](http://hl7.org/fhir/StructureDefinition/Timing) v5.0.0**

 A timing schedule that specifies an event that may occur multiple times

 Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.

 Describes the occurrence of an event that may occur multiple times. Timing schedules are used for specifying when events are expected or requested to occur, and may also be used to represent the summary of a past or ongoing event.  For simplicity, the definitions of Timing components are expressed as 'future' events, but such components can also be used to describe historic or ongoing events.

A Timing schedule can be a list of events and/or criteria for when the event happens, which can be expressed in a structured form and/or as a code. When both event and a repeating specification are provided, the list of events should be understood as an interpretation of the information in the repeat structure. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timing(pub Box<TimingInner>);
/** Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.

 **[Timing](http://hl7.org/fhir/StructureDefinition/Timing) v5.0.0**

 A timing schedule that specifies an event that may occur multiple times

 Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.

 Describes the occurrence of an event that may occur multiple times. Timing schedules are used for specifying when events are expected or requested to occur, and may also be used to represent the summary of a past or ongoing event.  For simplicity, the definitions of Timing components are expressed as 'future' events, but such components can also be used to describe historic or ongoing events.

A Timing schedule can be a list of events and/or criteria for when the event happens, which can be expressed in a structured form and/or as a code. When both event and a repeating specification are provided, the list of events should be understood as an interpretation of the information in the repeat structure. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "TimingBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct TimingInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Extensions that cannot be ignored even if unrecognized**

 May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.

Modifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself).

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Vec<Extension>,
    /** **When the event occurs**

 Identifies specific times when the event occurs.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "event")]
    pub event: Vec<Option<DateTime>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_event")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub event_ext: Vec<Option<FieldExtension>>,
    /** **When the event is to occur**

 A set of rules that describe when the event is scheduled.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "repeat")]
    pub repeat: Option<TimingRepeat>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_repeat")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub repeat_ext: Option<FieldExtension>,
    /** **[TimingAbbreviation](http://hl7.org/fhir/ValueSet/timing-abbreviation); C | BID | TID | QID | AM | PM | QD | QOD | +**

 A code for the timing schedule (or just text in code.text). Some codes such as BID are ubiquitous, but many institutions define their own additional codes. If a code is provided, the code is understood to be a complete statement of whatever is specified in the structured timing data, and either the code or the data may be used to interpret the Timing, with the exception that .repeat.bounds still applies over the code (and is not contained in the code).

 BID etc. are defined as 'at institutionally specified times'. For example, an institution may choose that BID is "always at 7am and 6pm".  If it is inappropriate for this choice to be made, the code BID should not be used. Instead, a distinct organization-specific code should be used in place of the HL7-defined BID code and/or a structured representation should be used (in this case, specifying the two event times). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl TimingBuilder {
    #[doc = concat!("Finalize building ", "Timing", ".")]
    pub fn build(self) -> Result<Timing, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<TimingInner> for Timing {
    fn from(inner: TimingInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Timing {
    type Target = TimingInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Timing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Timing {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> TimingBuilder {
        TimingBuilder::default()
    }
}
/// Sub-fields of the repeat field in Timing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "TimingRepeatBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct TimingRepeat {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Length/Range of lengths, or (Start and/or end) limits**

 Either a duration for the length of the timing schedule, a range of possible length, or outer bounds for start and/or end limits of the timing schedule.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub bounds: Option<TimingRepeatBounds>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub bounds_ext: Option<TimingRepeatBoundsExtension>,
    /** **Number of times to repeat**

 A total count of the desired number of repetitions across the duration of the entire timing specification. If countMax is present, this element indicates the lower bound of the allowed range of count values.

 If you have both bounds and count, then this should be understood as within the bounds period, until count times happens. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "count")]
    pub count: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_count")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub count_ext: Option<FieldExtension>,
    /** **Maximum number of times to repeat**

 If present, indicates that the count is a range - so to perform the action between [count] and [countMax] times.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "countMax")]
    pub count_max: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_countMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub count_max_ext: Option<FieldExtension>,
    /** **How long when it happens**

 How long this thing happens for when it happens. If durationMax is present, this element indicates the lower bound of the allowed range of the duration.

 For some events the duration is part of the definition of the event (e.g. IV infusions, where the duration is implicit in the specified quantity and rate). For others, it's part of the timing specification (e.g. exercise). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_duration")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_ext: Option<FieldExtension>,
    /** **How long when it happens (Max)**

 If present, indicates that the duration is a range - so to perform the action between [duration] and [durationMax] time length.

 For some events the duration is part of the definition of the event (e.g. IV infusions, where the duration is implicit in the specified quantity and rate). For others, it's part of the timing specification (e.g. exercise). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "durationMax")]
    pub duration_max: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_durationMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_max_ext: Option<FieldExtension>,
    /** **[UnitsOfTime](http://hl7.org/fhir/ValueSet/units-of-time); s | min | h | d | wk | mo | a - unit of time (UCUM)**

 The units of time for the duration, in UCUM units
Normal practice is to use the 'mo' code as a calendar month when calculating the next occurrence.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "durationUnit")]
    pub duration_unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_durationUnit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_unit_ext: Option<FieldExtension>,
    /** **Indicates the number of repetitions that should occur within a period. I.e. Event occurs frequency times per period**

 The number of times to repeat the action within the specified period. If frequencyMax is present, this element indicates the lower bound of the allowed range of the frequency.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "frequency")]
    pub frequency: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_frequency")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub frequency_ext: Option<FieldExtension>,
    /** **Event occurs up to frequencyMax times per period**

 If present, indicates that the frequency is a range - so to repeat between [frequency] and [frequencyMax] times within the period or period range.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "frequencyMax")]
    pub frequency_max: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_frequencyMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub frequency_max_ext: Option<FieldExtension>,
    /** **The duration to which the frequency applies. I.e. Event occurs frequency times per period**

 Indicates the duration of time over which repetitions are to occur; e.g. to express "3 times per day", 3 would be the frequency and "1 day" would be the period. If periodMax is present, this element indicates the lower bound of the allowed range of the period length.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
    /** **Upper limit of period (3-4 hours)**

 If present, indicates that the period is a range from [period] to [periodMax], allowing expressing concepts such as "do this once every 3-5 days.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "periodMax")]
    pub period_max: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_periodMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_max_ext: Option<FieldExtension>,
    /** **[UnitsOfTime](http://hl7.org/fhir/ValueSet/units-of-time); s | min | h | d | wk | mo | a - unit of time (UCUM)**

 The units of time for the period in UCUM units
Normal practice is to use the 'mo' code as a calendar month when calculating the next occurrence.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "periodUnit")]
    pub period_unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_periodUnit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_unit_ext: Option<FieldExtension>,
    /** **[DayOfWeek](http://hl7.org/fhir/ValueSet/days-of-week); mon | tue | wed | thu | fri | sat | sun**

 If one or more days of week is provided, then the action happens only on the specified day(s).

 If no days are specified, the action is assumed to happen every day as otherwise specified. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: Vec<Option<codes::DaysOfWeek>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_dayOfWeek")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub day_of_week_ext: Vec<Option<FieldExtension>>,
    /** **Time of day for action**

 Specified time of day for action to take place.

 When time of day is specified, it is inferred that the action happens every day (as filtered by dayofWeek) on the specified times. If there's a timeOfDay, there cannot be a when, or vice versa. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "timeOfDay")]
    pub time_of_day: Vec<Option<Time>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_timeOfDay")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub time_of_day_ext: Vec<Option<FieldExtension>>,
    /** **[EventTiming](http://hl7.org/fhir/ValueSet/event-timing); Code for time period of occurrence**

 An approximate time period during the day, potentially linked to an event of daily living that indicates when the action should occur.

 When more than one event is listed, the event is tied to the union of the specified events. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "when")]
    pub when: Vec<Option<codes::EventTiming>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_when")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub when_ext: Vec<Option<FieldExtension>>,
    /** **Minutes from event (before or after)**

 The number of minutes from the event. If the event code does not indicate whether the minutes is before or after the event, then the offset is assumed to be after the event.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "offset")]
    pub offset: Option<u32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_offset")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub offset_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl TimingRepeat {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> TimingRepeatBuilder {
        TimingRepeatBuilder::default()
    }
}
/// Choice of types for the bounds[x] field in TimingRepeat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimingRepeatBounds {
    /// Variant accepting the Duration type.
    #[serde(rename = "boundsDuration")]
    Duration(Duration),
    /// Variant accepting the Range type.
    #[serde(rename = "boundsRange")]
    Range(Range),
    /// Variant accepting the Period type.
    #[serde(rename = "boundsPeriod")]
    Period(Period),
}
/// Extension value for TimingRepeatBounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimingRepeatBoundsExtension {
    /// Variant accepting the Duration extension.
    #[serde(rename = "_boundsDuration")]
    Duration(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_boundsRange")]
    Range(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_boundsPeriod")]
    Period(FieldExtension),
}
/** TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.

 **[TriggerDefinition](http://hl7.org/fhir/StructureDefinition/TriggerDefinition) v5.0.0**

 Defines an expected trigger for a module

 A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.

 If an event is a named-event, it means the event is completely pre-coordinated, and no other information can be specified for the event. If the event is one of the data- events, the data and condition elements specify the triggering criteria. The data element specifies the structured component, and the condition element provides additional optional refinement of that structured component. If the event is periodic, the timing element defines when the event is triggered. For both data- and periodic events, a name can be provided as a shorthand for the formal semantics provided by the other elements. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TriggerDefinition(pub Box<TriggerDefinitionInner>);
/** TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.

 **[TriggerDefinition](http://hl7.org/fhir/StructureDefinition/TriggerDefinition) v5.0.0**

 Defines an expected trigger for a module

 A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.

 If an event is a named-event, it means the event is completely pre-coordinated, and no other information can be specified for the event. If the event is one of the data- events, the data and condition elements specify the triggering criteria. The data element specifies the structured component, and the condition element provides additional optional refinement of that structured component. If the event is periodic, the timing element defines when the event is triggered. For both data- and periodic events, a name can be provided as a shorthand for the formal semantics provided by the other elements. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "TriggerDefinitionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct TriggerDefinitionInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[TriggerType](http://hl7.org/fhir/ValueSet/trigger-type); named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended**

 The type of triggering event.

 */
    #[serde(rename = "type")]
    pub r#type: codes::TriggerType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Name or URI that identifies the event**

 A formal name for the event. This may be an absolute URI that identifies the event formally (e.g. from a trigger registry), or a simple relative URI that identifies the event in a local context.

 An event name can be provided for all event types, but is required for named events. If a name is provided for a type other than named events, it is considered to be a shorthand for the semantics described by the formal description of the event. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **Coded definition of the event**

 A code that identifies the event.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **What event**

 A reference to a SubscriptionTopic resource that defines the event. If this element is provided, no other information about the trigger definition may be supplied.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "subscriptionTopic")]
    pub subscription_topic: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_subscriptionTopic")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub subscription_topic_ext: Option<FieldExtension>,
    /** **Timing of the event**

 The timing of the event (if this is a periodic trigger).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub timing: Option<TriggerDefinitionTiming>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub timing_ext: Option<TriggerDefinitionTimingExtension>,
    /** **Triggering data of the event (multiple = 'and')**

 The triggering data of the event (if this is a data trigger). If more than one data is requirement is specified, then all the data requirements must be true.

 This element shall be present for any data type trigger. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "data")]
    pub data: Vec<Option<DataRequirement>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_data")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub data_ext: Vec<Option<FieldExtension>>,
    /** **Whether the event triggers (boolean expression)**

 A boolean-valued expression that is evaluated in the context of the container of the trigger definition and returns whether or not the trigger fires.

 This element can be only be specified for data type triggers and provides additional semantics for the trigger. The context available within the condition is based on the type of data event. For all events, the current resource will be available as context. In addition, for modification events, the previous resource will also be available. The expression may be inlined, or may be a simple absolute URI, which is a reference to a named expression within a logic library referenced by a library element or extension within the containing resource. If the expression is a FHIR Path expression, it evaluates in the context of a resource of one of the type identified in the data requirement, and may also refer to the variable %previous for delta comparisons on events of type data-changed, data-modified, and data-deleted which will always have the same type. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "condition")]
    pub condition: Option<Expression>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_condition")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub condition_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl TriggerDefinitionBuilder {
    #[doc = concat!("Finalize building ", "TriggerDefinition", ".")]
    pub fn build(self) -> Result<TriggerDefinition, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<TriggerDefinitionInner> for TriggerDefinition {
    fn from(inner: TriggerDefinitionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for TriggerDefinition {
    type Target = TriggerDefinitionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for TriggerDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TriggerDefinition {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> TriggerDefinitionBuilder {
        TriggerDefinitionBuilder::default()
    }
}
/// Choice of types for the timing[x] field in TriggerDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerDefinitionTiming {
    /// Variant accepting the Timing type.
    #[serde(rename = "timingTiming")]
    Timing(Timing),
    /// Variant accepting the Reference type.
    #[serde(rename = "timingReference")]
    Reference(Reference),
    /// Variant accepting the Date type.
    #[serde(rename = "timingDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "timingDateTime")]
    DateTime(DateTime),
}
/// Extension value for TriggerDefinitionTiming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerDefinitionTimingExtension {
    /// Variant accepting the Timing extension.
    #[serde(rename = "_timingTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_timingReference")]
    Reference(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_timingDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_timingDateTime")]
    DateTime(FieldExtension),
}
/** UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).

 **[UsageContext](http://hl7.org/fhir/StructureDefinition/UsageContext) v5.0.0**

 Describes the context of use for a conformance or knowledge resource

 Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UsageContext(pub Box<UsageContextInner>);
/** UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).

 **[UsageContext](http://hl7.org/fhir/StructureDefinition/UsageContext) v5.0.0**

 Describes the context of use for a conformance or knowledge resource

 Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "UsageContextBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct UsageContextInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[UsageContextType](http://terminology.hl7.org/ValueSet/usage-context-type); Type of context being specified**

 A code that identifies the type of context being specified by this usage context.

 */
    #[serde(rename = "code")]
    pub code: Coding,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Value that defines the context**

 A value that defines the context specified in this context of use. The interpretation of the value is defined by the code.

 */
    #[serde(flatten)]
    pub value: UsageContextValue,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<UsageContextValueExtension>,
}
#[cfg(feature = "builders")]
impl UsageContextBuilder {
    #[doc = concat!("Finalize building ", "UsageContext", ".")]
    pub fn build(self) -> Result<UsageContext, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<UsageContextInner> for UsageContext {
    fn from(inner: UsageContextInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for UsageContext {
    type Target = UsageContextInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for UsageContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl UsageContext {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> UsageContextBuilder {
        UsageContextBuilder::default()
    }
}
/// Choice of types for the value[x] field in UsageContext
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UsageContextValue {
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "valueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Quantity type.
    #[serde(rename = "valueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "valueRange")]
    Range(Range),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueReference")]
    Reference(Reference),
}
/// Extension value for UsageContextValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UsageContextValueExtension {
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_valueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_valueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_valueRange")]
    Range(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueReference")]
    Reference(FieldExtension),
}
/** VirtualServiceDetail Type: Virtual Service Contact Details.

 **[VirtualServiceDetail](http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail) v5.0.0**

 Virtual Service Contact Details

 Virtual Service Contact Details.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VirtualServiceDetail(pub Box<VirtualServiceDetailInner>);
/** VirtualServiceDetail Type: Virtual Service Contact Details.

 **[VirtualServiceDetail](http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail) v5.0.0**

 Virtual Service Contact Details

 Virtual Service Contact Details.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "VirtualServiceDetailBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct VirtualServiceDetailInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[VirtualServiceType](http://hl7.org/fhir/ValueSet/virtual-service-type); Channel Type**

 The type of virtual service to connect to (i.e. Teams, Zoom, Specific VMR technology, WhatsApp).

 As this type defines what the virtual service is, it then defines what address and additional information might be relevant.

Some profiles could be used for specific types to define what the other values could/should be. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "channelType")]
    pub channel_type: Option<Coding>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_channelType")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub channel_type_ext: Option<FieldExtension>,
    /** **Contact address/number**

 What address or number needs to be used for a user to connect to the virtual service to join. The channelType informs as to which datatype is appropriate to use (requires knowledge of the specific type).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub address: Option<VirtualServiceDetailAddress>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub address_ext: Option<VirtualServiceDetailAddressExtension>,
    /** **Address to see alternative connection details**

 Address to see alternative connection details.

 This web address can be used to provide additional details on the call, such as alternative/regional call in numbers, or other associated services. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "additionalInfo")]
    pub additional_info: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_additionalInfo")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub additional_info_ext: Vec<Option<FieldExtension>>,
    /** **Maximum number of participants supported by the virtual service**

 Maximum number of participants supported by the virtual service.

 Some services have only a limitted number of participants permitted to connect at a time, this is typically used in "shared" virtual services that you might find on location resources.

Typically on-demand services might not have these restrictions. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxParticipants")]
    pub max_participants: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxParticipants")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_participants_ext: Option<FieldExtension>,
    /** **Session Key required by the virtual service**

 Session Key required by the virtual service.

 Some services require a session key to be able to access the service once connected.

This could be in cases where a shared number is used, and a session key is added to put into a private line, or to identify the participant in a call. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sessionKey")]
    pub session_key: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sessionKey")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub session_key_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl VirtualServiceDetailBuilder {
    #[doc = concat!("Finalize building ", "VirtualServiceDetail", ".")]
    pub fn build(self) -> Result<VirtualServiceDetail, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<VirtualServiceDetailInner> for VirtualServiceDetail {
    fn from(inner: VirtualServiceDetailInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for VirtualServiceDetail {
    type Target = VirtualServiceDetailInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for VirtualServiceDetail {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl VirtualServiceDetail {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> VirtualServiceDetailBuilder {
        VirtualServiceDetailBuilder::default()
    }
}
/// Choice of types for the address[x] field in VirtualServiceDetail
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VirtualServiceDetailAddress {
    /// Variant accepting the Url type.
    #[serde(rename = "addressUrl")]
    Url(String),
    /// Variant accepting the String type.
    #[serde(rename = "addressString")]
    String(String),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "addressContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the ExtendedContactDetail type.
    #[serde(rename = "addressExtendedContactDetail")]
    ExtendedContactDetail(ExtendedContactDetail),
}
/// Extension value for VirtualServiceDetailAddress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VirtualServiceDetailAddressExtension {
    /// Variant accepting the Url extension.
    #[serde(rename = "_addressUrl")]
    Url(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_addressString")]
    String(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_addressContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the ExtendedContactDetail extension.
    #[serde(rename = "_addressExtendedContactDetail")]
    ExtendedContactDetail(FieldExtension),
}
/** An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)

 **[MoneyQuantity](http://hl7.org/fhir/StructureDefinition/MoneyQuantity) v5.0.0**

 An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)

 There SHALL be a code if there is a value and it SHALL be an expression of currency.  If system is present, it SHALL be ISO 4217 (system = "urn:iso:std:iso:4217" - currency).

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MoneyQuantity(pub Box<MoneyQuantityInner>);
/** An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)

 **[MoneyQuantity](http://hl7.org/fhir/StructureDefinition/MoneyQuantity) v5.0.0**

 An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)

 There SHALL be a code if there is a value and it SHALL be an expression of currency.  If system is present, it SHALL be ISO 4217 (system = "urn:iso:std:iso:4217" - currency).

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MoneyQuantityBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MoneyQuantityInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comparator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comparator_ext: Option<FieldExtension>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_unit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub unit_ext: Option<FieldExtension>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl MoneyQuantityBuilder {
    #[doc = concat!("Finalize building ", "MoneyQuantity", ".")]
    pub fn build(self) -> Result<MoneyQuantity, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MoneyQuantityInner> for MoneyQuantity {
    fn from(inner: MoneyQuantityInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for MoneyQuantity {
    type Target = MoneyQuantityInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for MoneyQuantity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl MoneyQuantity {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MoneyQuantityBuilder {
        MoneyQuantityBuilder::default()
    }
}
/** A fixed quantity (no comparator)

 **[SimpleQuantity](http://hl7.org/fhir/StructureDefinition/SimpleQuantity) v5.0.0**

 A fixed quantity (no comparator)

 The comparator is not used on a SimpleQuantity

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SimpleQuantity(pub Box<SimpleQuantityInner>);
/** A fixed quantity (no comparator)

 **[SimpleQuantity](http://hl7.org/fhir/StructureDefinition/SimpleQuantity) v5.0.0**

 A fixed quantity (no comparator)

 The comparator is not used on a SimpleQuantity

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "SimpleQuantityBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct SimpleQuantityInner {
    /** **Unique id for inter-element referencing**

 Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > | ad - how to understand the value**

 Not allowed to be used in this context

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Vec<Option<codes::QuantityComparator>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_comparator")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub comparator_ext: Vec<Option<FieldExtension>>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_unit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub unit_ext: Option<FieldExtension>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl SimpleQuantityBuilder {
    #[doc = concat!("Finalize building ", "SimpleQuantity", ".")]
    pub fn build(self) -> Result<SimpleQuantity, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<SimpleQuantityInner> for SimpleQuantity {
    fn from(inner: SimpleQuantityInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for SimpleQuantity {
    type Target = SimpleQuantityInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for SimpleQuantity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl SimpleQuantity {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> SimpleQuantityBuilder {
        SimpleQuantityBuilder::default()
    }
}
/// Extension of a field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "FieldExtensionBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct FieldExtension {
    /// Unique id for inter-element referencing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub extension: Vec<Extension>,
}
#[cfg(feature = "builders")]
impl FieldExtension {
    ///Start building a new FieldExtension.
    #[must_use]
    pub fn builder() -> FieldExtensionBuilder {
        FieldExtensionBuilder::default()
    }
}
