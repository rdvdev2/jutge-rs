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

/// Type-safe representation of a (non-localized) problem id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProblemId([u8; 7]);

impl ProblemId {
    /// Creates a `ProblemId` from a type and a numeric id.
    ///
    /// # Errors
    /// The numeric id must be at most 6 digits long. If it isn't,
    /// [`Error::InvalidProblemId`] will be returned.
    pub fn new(pt: ProblemType, id: u32) -> Result<Self> {
        if id < 1_000_000 {
            Ok(Self(
                format!("{}{:06}", pt.letter(), id)
                    .into_bytes()
                    .try_into()
                    .expect("String should be 7 bytes long!"),
            ))
        } else {
            Err(Error::InvalidProblemId(
                "numeric id must be at most 6 digits long".into(),
            ))
        }
    }

    /// Gets the `ProblemType` of the problem id
    #[must_use]
    pub fn problem_type(&self) -> ProblemType {
        (self.0[0] as char)
            .try_into()
            .expect("Problem ID first char should represent a Problem Type!")
    }

    /// Gets the numeric id of the problem id
    #[must_use]
    pub fn problem_id(&self) -> u32 {
        self.0
            .iter()
            .map(|x| x - b'0')
            .fold(0, |acc, x| acc * 10 + u32::from(x))
    }
}

impl FromStr for ProblemId {
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

        Self::new(pt, id)
    }
}

impl Display for ProblemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from_utf8_lossy(&self.0).as_ref())
    }
}
