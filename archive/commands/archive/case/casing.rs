
pub use case::Case;
pub use conversion::Converter;
pub use pattern::Pattern;
pub use segmentation::Boundary;

/// Describes items that can be converted into a case.  This trait is used
/// in conjunction with the [`StateConverter`] struct which is returned from a couple
/// methods on `Casing`.
///
/// Implemented for strings `&str`, `String`, and `&String`.
pub trait Casing<T: AsRef<str>> {
    /// Convert the string into the given case.  It will reference `self` and create a new
    /// `String` with the same pattern and delimeter as `case`.  It will split on boundaries
    /// defined at [`Boundary::defaults()`].
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "tetronimo-piece-border",
    ///     "Tetronimo piece border".to_case(Case::Kebab)
    /// );
    /// ```
    fn to_case(&self, case: Case) -> String;

    /// Start the case conversion by storing the boundaries associated with the given case.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "2020-08-10_dannie_birthday",
    ///     "2020-08-10 Dannie Birthday"
    ///         .from_case(Case::Title)
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn from_case(&self, case: Case) -> StateConverter<T>;

    /// Creates a `StateConverter` struct initialized with the boundaries
    /// provided.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "e1_m1_hangar",
    ///     "E1M1 Hangar"
    ///         .with_boundaries(&[Boundary::DigitUpper, Boundary::Space])
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<T>;

    /// Determines if `self` is of the given case.  This is done simply by applying
    /// the conversion and seeing if the result is the same.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert!( "kebab-case-string".is_case(Case::Kebab));
    /// assert!( "Train-Case-String".is_case(Case::Train));
    ///
    /// assert!(!"kebab-case-string".is_case(Case::Snake));
    /// assert!(!"kebab-case-string".is_case(Case::Train));
    /// ```
    fn is_case(&self, case: Case) -> bool;
}

impl<T: AsRef<str>> Casing<T> for T
where
    String: PartialEq<T>,
{
    fn to_case(&self, case: Case) -> String {
        StateConverter::new(self).to_case(case)
    }

    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<T> {
        StateConverter::new(self).with_boundaries(bs)
    }

    fn from_case(&self, case: Case) -> StateConverter<T> {
        StateConverter::new_from_case(self, case)
    }

    fn is_case(&self, case: Case) -> bool {
        &self.to_case(case) == self
    }
}

/// Holds information about parsing before converting into a case.
///
/// This struct is used when invoking the `from_case` and `with_boundaries` methods on
/// `Casing`.  For a more fine grained approach to case conversion, consider using the [`Converter`]
/// struct.
/// ```
/// use convert_case::{Case, Casing};
///
/// let title = "ninety-nine_problems".from_case(Case::Snake).to_case(Case::Title);
/// assert_eq!("Ninety-nine Problems", title);
/// ```
pub struct StateConverter<'a, T: AsRef<str>> {
    s: &'a T,
    conv: Converter,
}

impl<'a, T: AsRef<str>> StateConverter<'a, T> {
    /// Only called by Casing function to_case()
    fn new(s: &'a T) -> Self {
        Self {
            s,
            conv: Converter::new(),
        }
    }

    /// Only called by Casing function from_case()
    fn new_from_case(s: &'a T, case: Case) -> Self {
        Self {
            s,
            conv: Converter::new().from_case(case),
        }
    }

    /// Uses the boundaries associated with `case` for word segmentation.  This
    /// will overwrite any boundary information initialized before.  This method is
    /// likely not useful, but provided anyway.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// let name = "Chuck Schuldiner"
    ///     .from_case(Case::Snake) // from Casing trait
    ///     .from_case(Case::Title) // from StateConverter, overwrites previous
    ///     .to_case(Case::Kebab);
    /// assert_eq!("chuck-schuldiner", name);
    /// ```
    pub fn from_case(self, case: Case) -> Self {
        Self {
            conv: self.conv.from_case(case),
            ..self
        }
    }

    /// Overwrites boundaries for word segmentation with those provided.  This will overwrite
    /// any boundary information initialized before.  This method is likely not useful, but
    /// provided anyway.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// let song = "theHumbling river-puscifer"
    ///     .from_case(Case::Kebab) // from Casing trait
    ///     .with_boundaries(&[Boundary::Space, Boundary::LowerUpper]) // overwrites `from_case`
    ///     .to_case(Case::Pascal);
    /// assert_eq!("TheHumblingRiver-puscifer", song);  // doesn't split on hyphen `-`
    /// ```
    pub fn with_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.set_boundaries(bs),
        }
    }

    /// Removes any boundaries that were already initialized.  This is particularly useful when a
    /// case like `Case::Camel` has a lot of associated word boundaries, but you want to exclude
    /// some.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "2d_transformation",
    ///     "2dTransformation"
    ///         .from_case(Case::Camel)
    ///         .without_boundaries(&Boundary::digits())
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    pub fn without_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.remove_boundaries(bs),
        }
    }

    /// Consumes the `StateConverter` and returns the converted string.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "ice-cream social",
    ///     "Ice-Cream Social".from_case(Case::Title).to_case(Case::Lower)
    /// );
    /// ```
    pub fn to_case(self, case: Case) -> String {
        self.conv.to_case(case).convert(self.s)
    }
}
