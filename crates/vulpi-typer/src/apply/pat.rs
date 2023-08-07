use im_rc::HashMap;
use vulpi_intern::Symbol;
use vulpi_location::Span;
use vulpi_syntax::r#abstract::Pattern;

use crate::{
    env::Env,
    types::{HoleInner, Type, TypeKind},
    Infer,
};

use super::Apply;

impl Apply for Pattern {
    type Return = Type;

    type Context<'a> = (Env, &'a mut HashMap<Symbol, (Span, Type)>);

    fn apply(&self, ty: crate::types::Type, (env, map): Self::Context<'_>) -> Self::Return {
        match ty.as_ref() {
            TypeKind::Hole(hole) => match hole.0.borrow().clone() {
                HoleInner::Filled(ty) => self.apply(ty, (env, map)),
                HoleInner::Empty(_, k) => {
                    let ret = env.new_hole(k.clone());
                    let e = env.new_hole(k);
                    let arg = self.infer((env, map));

                    hole.0
                        .replace(HoleInner::Filled(Type::arrow(arg, e, ret.clone())));

                    ret
                }
                HoleInner::Lacks(_) => unreachable!(),
            },
            TypeKind::Arrow(l, _, r) => {
                let arg = self.infer((env.clone(), map));

                Type::unify(env, arg, l.clone());
                r.clone()
            }
            TypeKind::Forall(p, k, t) => {
                self.apply(t.instantiate(env.clone(), p.clone(), k.clone()), (env, map))
            }
            TypeKind::Error => Type::error(),
            _ => {
                env.report(crate::error::TypeErrorKind::NotAFunction(
                    env.clone(),
                    ty.clone(),
                ));

                Type::error()
            }
        }
    }
}
