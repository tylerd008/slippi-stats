use crate::gamedata::GameData;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

//these traits should probably be somewhere else but i'm not sure where yet
pub trait Numbered {
    const NUM_VALUES: usize;
}
pub trait Parsable: FromStr + Display + TryFrom<usize> {}
pub trait UnnamedTrait {
    //name this
    fn condition(&self, game: &GameData) -> bool; //come up with more descriptive name
}

#[derive(Debug)]
pub enum ParsableEnumError {
    FromStrError,
    TryFromError,
}

#[macro_export]
macro_rules! parsable_enum {
    ($vis:vis enum $name:ident{
        $($disp_name:literal; $($alias:literal,)* => $val:ident = $num_val:expr,)*
    }) => {
        use crate::parsable_enum::ParsableEnumError;
        #[derive(Clone, Copy, Debug, PartialEq)]
        $vis enum $name {
            $($val = $num_val,)*
        }

        impl crate::parsable_enum::Parsable for $name{}

        impl std::str::FromStr for $name {
            type Err = ParsableEnumError;
            fn from_str(arg: &str) -> Result<Self, Self::Err> {
                match &(arg.to_string())[..]{
                    $(x if x == $disp_name.to_lowercase() => Ok($name::$val),
                    $($alias => Ok($name::$val),)*)*
                _ => Err(ParsableEnumError::FromStrError)
                }
            }
        }

        impl std::convert::TryFrom<usize> for $name{
            type Error = ParsableEnumError;
            fn try_from(num: usize) -> Result<Self, Self::Error> {
                match num {
                    $(x if x == $name::$val as usize => Ok($name::$val),)*
                    _ => Err(ParsableEnumError::TryFromError)
                }
            }
        }

        impl std::fmt::Display for $name{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let text = match self{
                    $($name::$val => $disp_name,)*
                };
                write!(f, "{}", text)
            }
        }
    }
}
