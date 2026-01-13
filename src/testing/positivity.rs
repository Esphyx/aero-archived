#[derive(Clone, Debug)]
pub enum ConstructorType {
    Inductive,
    Independent,
    Arrow(Box<ConstructorType>, Box<ConstructorType>),
}

fn strictly_positive(ty: &ConstructorType, positive: bool) -> bool {
    match ty {
        ConstructorType::Inductive => positive, // type being inductively defined
        ConstructorType::Independent => true,   // another independent type
        ConstructorType::Arrow(arg, out) => {
            strictly_positive(arg, !positive) && strictly_positive(out, positive)
        }
    }
}

fn check_positivity(ty: &ConstructorType) -> bool {
    strictly_positive(ty, true)
}

#[cfg(test)]
mod tests {
    use crate::positivity::{ConstructorType, check_positivity};

    fn arrow(arg: &ConstructorType, out: &ConstructorType) -> ConstructorType {
        ConstructorType::Arrow(Box::new(arg.clone()), Box::new(out.clone()))
    }

    #[test]
    fn test() {
        let t = ConstructorType::Inductive;
        let a = ConstructorType::Independent;

        // let t_t = arrow(&t, &t);
        let a_t = arrow(&t, &a);
        let t_a = arrow(&t, &a);

        let a_t__t = arrow(&a_t, &t);
        // let a_t__t_t = arrow(&a_t, &t_t);

        let a_t__t___t = arrow(&a_t__t, &t);

        assert_eq!(check_positivity(&t), true);
        // assert_eq!(check_positivity(&t_t), true);
        assert_eq!(check_positivity(&a_t__t), true);
        assert_eq!(check_positivity(&a_t__t___t), false);
        // assert_eq!(check_positivity(&a_t__t_t), true);
    }
}
