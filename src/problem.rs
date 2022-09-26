use std::{convert::TryInto, fmt::Display, str::FromStr};

use crate::{Error, Result};

/// Represents a <https://jutge.org> problem type.
///
/// Officially, the problem type is indicated by the letter in the problem id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProblemType {
    /// A Game problem (G).
    ///
    /// Game problems are publicly accessible. See [ProblemType::Public] for more information.
    Game,

    /// A public problem (P).
    ///
    /// Public problems can be accessed without authenticating to
    /// <https://jutge.org>.
    Public,

    /// A private problem (X).
    ///
    /// Private problems can't be accessed without authenticating to
    /// <https://jutge.org>. There's a couple of facts to consider when
    /// working with private problems:
    ///  - Not all users have access to all private problems.
    ///  - It's impossible to differenciate between a non-existing problem
    ///    and a non-accessible problem.
    Private,
}

impl ProblemType {
    /// Get the letter representing this problem type in problem ids.
    #[must_use]
    pub const fn letter(&self) -> char {
        match self {
            ProblemType::Game => 'G',
            ProblemType::Public => 'P',
            ProblemType::Private => 'X',
        }
    }

    /// Checks if a letter represents a problem type.
    #[must_use]
    pub const fn is_valid_letter(letter: char) -> bool {
        matches!(letter, 'G' | 'P' | 'X')
    }
}

impl From<ProblemType> for char {
    fn from(pt: ProblemType) -> Self {
        pt.letter()
    }
}

impl TryFrom<char> for ProblemType {
    type Error = Error;

    fn try_from(val: char) -> Result<Self> {
        match val {
            'G' => Ok(Self::Game),
            'P' => Ok(Self::Public),
            'X' => Ok(Self::Private),
            _ => Err(Error::NotAProblemType),
        }
    }
}

/// The possible languages for a <https://jutge.org> problem.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum ProblemLanguage {
    /// The Catalan language (ca)
    Catalan,

    /// The English language (en)
    English,

    /// The Spanish language (es)
    Spanish,

    /// The French language (fr)
    French,

    /// The German language (de)
    German,
}

impl ProblemLanguage {
    /// Returns the 2 letter code of the language as a 2 byte array
    #[must_use]
    pub const fn code(&self) -> [u8; 2] {
        use const_str::as_bytes;

        match self {
            Self::Catalan => *as_bytes!("ca"),
            Self::English => *as_bytes!("en"),
            Self::Spanish => *as_bytes!("es"),
            Self::French => *as_bytes!("fr"),
            Self::German => *as_bytes!("de"),
        }
    }
}

impl TryFrom<[u8; 2]> for ProblemLanguage {
    type Error = Error;

    fn try_from(value: [u8; 2]) -> Result<Self> {
        use const_str::as_bytes;

        match &value {
            as_bytes!("ca") => Ok(Self::Catalan),
            as_bytes!("en") => Ok(Self::English),
            as_bytes!("es") => Ok(Self::Spanish),
            as_bytes!("fr") => Ok(Self::French),
            as_bytes!("de") => Ok(Self::German),
            _ => Err(Error::NotAProblemLanguage),
        }
    }
}

impl TryFrom<&[u8]> for ProblemLanguage {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 2 {
            return Err(Error::NotAProblemLanguage);
        }
        let code = [value[0], value[1]];
        code.try_into()
    }
}

/// Contains the different types of [`ProblemId`]'s
pub mod problem_id_types {
    use sealed::sealed;

    /// This trait allows the type system to distinguish localized and unlocalized
    /// problem ids.
    #[sealed]
    pub trait ProblemIdType {
        /// Returns an slice of the UTF-8 representation of the problem id.
        fn representation(&self) -> &[u8];
    }

    /// Used on non-localized problem ids
    pub struct Unlocalized(pub(super) [u8; 7]);

    #[sealed]
    impl ProblemIdType for Unlocalized {
        fn representation(&self) -> &[u8] {
            &self.0
        }
    }

    /// Used on localized problem ids
    pub struct Localized(pub(super) [u8; 10]);

    #[sealed]
    impl ProblemIdType for Localized {
        fn representation(&self) -> &[u8] {
            &self.0
        }
    }
}

#[allow(clippy::wildcard_imports)]
use problem_id_types::*;

/// Type-safe representation of a problem id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProblemId<T: ProblemIdType>(T);

impl<T: ProblemIdType> ProblemId<T> {
    /// Gets the `ProblemType` of the problem id
    #[must_use]
    pub fn problem_type(&self) -> ProblemType {
        (self.0.representation()[0] as char)
            .try_into()
            .expect("Problem ID first char should represent a Problem Type!")
    }

    /// Gets the numeric id of the problem id
    #[must_use]
    pub fn problem_id(&self) -> u32 {
        self.0
            .representation()
            .iter()
            .map(|x| x - b'0')
            .fold(0, |acc, x| acc * 10 + u32::from(x))
    }
}

impl<T: ProblemIdType> Display for ProblemId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from_utf8_lossy(self.0.representation()).as_ref())
    }
}

impl ProblemId<Unlocalized> {
    /// Creates an unlocalized `ProblemId` from a type and a numeric id.
    ///
    /// # Errors
    /// The numeric id must be at most 6 digits long. If it isn't,
    /// [`Error::InvalidProblemId`] will be returned.
    pub fn new_unlocalized(pt: ProblemType, id: u32) -> Result<Self> {
        if id < 1_000_000 {
            Ok(Self(Unlocalized(
                format!("{}{:06}", pt.letter(), id)
                    .into_bytes()
                    .try_into()
                    .expect("String should be 7 bytes long!"),
            )))
        } else {
            Err(Error::InvalidProblemId(
                "numeric id must be at most 6 digits long".into(),
            ))
        }
    }
}

impl FromStr for ProblemId<Unlocalized> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 7 {
            return Err(Error::InvalidProblemId(
                "string should be 7 characters long".into(),
            ));
        }

        let pt = s.chars().next().unwrap().try_into()?;
        let id = s[1..=6]
            .parse()
            .map_err(|_| Error::InvalidProblemId("last 6 characters should be numeric".into()))?;

        Self::new_unlocalized(pt, id)
    }
}

impl ProblemId<Localized> {
    /// Creates a localized `ProblemId` from a type, a numeric id and a language.
    ///
    /// # Errors
    /// The numeric id must be at most 6 digits long. If it isn't,
    /// [`Error::InvalidProblemId`] will be returned.
    pub fn new_localized(pt: ProblemType, id: u32, lang: ProblemLanguage) -> Result<Self> {
        let internal = {
            let mut internal = [0; 10];

            let unlocalized = ProblemId::new_unlocalized(pt, id)?.0;
            internal[0..=6].clone_from_slice(unlocalized.0.as_slice());

            internal[7] = b'_';
            internal[8..=9].clone_from_slice(lang.code().as_slice());

            internal
        };

        Ok(Self(Localized(internal)))
    }

    /// Gets the `ProblemLanguage` of the `ProblemId`
    #[must_use]
    pub fn language(&self) -> ProblemLanguage {
        let code = &self.0.representation()[8..=9];
        let code = [code[0], code[1]];
        code.try_into()
            .expect("Problem id last two chars should represent a problem language")
    }
}

impl FromStr for ProblemId<Localized> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 10 {
            return Err(Error::InvalidProblemId(
                "string should be 10 characters long".into(),
            ));
        }

        if s.as_bytes()[7] != b'_' {
            return Err(Error::InvalidProblemId(
                "unexpected character in problem id".into(),
            ));
        }

        let pt = s.chars().next().unwrap().try_into()?;
        let id = s[1..=6]
            .parse()
            .map_err(|_| Error::InvalidProblemId("last 6 characters should be numeric".into()))?;
        let lang = s.as_bytes()[8..=9].try_into()?;

        Self::new_localized(pt, id, lang)
    }
}
