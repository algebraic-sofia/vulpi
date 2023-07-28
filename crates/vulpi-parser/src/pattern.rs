use vulpi_location::Spanned;
use vulpi_syntax::{
    concrete::{pattern::*, Either},
    tokens::TokenData,
};

use crate::{Parser, Result};

impl<'a> Parser<'a> {
    pub fn pattern_atom_kind(&mut self) -> Result<PatternKind> {
        match self.token() {
            TokenData::Wildcard => Ok(PatternKind::Wildcard(self.bump())),
            TokenData::LowerIdent => self.lower().map(PatternKind::Variable),
            TokenData::UpperIdent => {
                let path = self.path_ident()?;
                match path.diferentiate() {
                    Either::Left(upper) => Ok(PatternKind::Constructor(upper)),
                    Either::Right(_) => todo!(),
                }
            }
            TokenData::LPar => self
                .parenthesis(Self::pattern)
                .map(PatternKind::Parenthesis),
            _ => self.literal().map(PatternKind::Literal),
        }
    }

    pub fn pattern_atom(&mut self) -> Result<Box<Pattern>> {
        self.spanned(Self::pattern_atom_kind).map(Box::new)
    }

    pub fn pattern_application_kind(&mut self) -> Result<PatApplication> {
        let func = self.path_upper()?;
        let args = self.many(Self::pattern_atom)?;
        Ok(PatApplication { func, args })
    }

    pub fn pattern_application(&mut self) -> Result<Box<Pattern>> {
        if self.at(TokenData::UpperIdent) {
            self.spanned(|this| {
                let result = this.pattern_application_kind()?;
                if result.args.is_empty() {
                    Ok(PatternKind::Constructor(result.func))
                } else {
                    Ok(PatternKind::Application(result))
                }
            })
            .map(Box::new)
        } else if self.at(TokenData::LBrace) {
            self.spanned(|this| {
                let left_brace = this.bump();
                let func = this.path_lower()?;
                let args = this.many(Self::pattern_atom)?;
                let right_brace = this.expect(TokenData::RBrace)?;
                Ok(PatternKind::EffectApp(PatEffectApp {
                    left_brace,
                    func,
                    args,
                    right_brace,
                }))
            })
            .map(Box::new)
        } else {
            self.pattern_atom()
        }
    }

    pub fn pattern(&mut self) -> Result<Box<Pattern>> {
        let left = self.pattern_application()?;
        if self.at(TokenData::Bar) {
            let pipe = self.bump();
            let right = self.pattern()?;
            Ok(Box::new(Spanned {
                span: left.span.clone().mix(right.span.clone()),
                data: PatternKind::Or(PatOr { left, pipe, right }),
            }))
        } else {
            Ok(left)
        }
    }
}
