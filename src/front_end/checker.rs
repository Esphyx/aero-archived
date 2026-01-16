// use std::{collections::HashMap, fmt::Display};

// use crate::ast::Expression;

// #[derive(Debug)]
// pub enum TypeError {
//     UnboundVariable(String),
//     TypeMismatch { expected: Expression, found: Expression },
//     NotAFunction(Expression),
//     ConditionalNotBool(Expression),
//     BrachesDontMatch { then_type: Expression, else_type: Expression },
// }

// impl Display for TypeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl std::error::Error for TypeError {}

// pub type Context = HashMap<String, Expression>;

// pub fn type_check(expression: &Expression, context: &Context) -> Result<Expression, TypeError> {
//     match expression {
//         Expression::Arrow { input, output } => {
//             todo!()
//         }
//         Expression::Variable(name) => context
//             .get(name)
//             .cloned()
//             .ok_or_else(|| TypeError::UnboundVariable(name.clone())),
//         Expression::Lambda {
//             parameter,
//             of_type,
//             body,
//         } => {
//             let mut extended_context = context.clone();
//             extended_context.insert(parameter.clone(), *of_type.clone()); // TODO: DE BRUIJN
//             let body_type = type_check(body, &extended_context)?;
//             Ok(Expression::Arrow {
//                 input: of_type.clone(),
//                 output: Box::new(body_type),
//             })
//         }
//         Expression::Application { function, argument } => {
//             let function_type = type_check(function, context)?;
//             let argument_type = type_check(argument, context)?;

//             match function_type {
//                 Expression::Arrow { input, output} => {
//                     if *input == argument_type {
//                         Ok(*output)
//                     } else {
//                         Err(TypeError::TypeMismatch {
//                             expected: *input,
//                             found: argument_type,
//                         })
//                     }
//                 }
//                 other => Err(TypeError::NotAFunction(other)),
//             }
//         } // Expression::Bool(_) => Ok(Type::Bool),
//           // Expression::If {
//           //     condition,
//           //     then_branch,
//           //     else_branch,
//           // } => {
//           //     let conditional_type = type_check(condition, context)?;
//           //     if conditional_type != Type::Bool {
//           //         return Err(TypeError::ConditionalNotBool(conditional_type));
//           //     }

//           //     let then_type = type_check(then_branch, context)?;
//           //     let else_type = type_check(else_branch, context)?;

//           //     if then_type == else_type {
//           //         Ok(then_type)
//           //     } else {
//           //         Err(TypeError::BrachesDontMatch {
//           //             then_type,
//           //             else_type,
//           //         })
//           //     }
//           // }
//           // Expression::LetBinding {
//           //     variable_name,
//           //     bound_to,
//           //     expression_after_binding,
//           // } => {
//           //     let t = type_check(bound_to, context)?;

//           //     let mut extended_context = context.clone();
//           //     extended_context.insert(variable_name.clone(), t); // TODO: DE BRUIJN
//           //     type_check(expression_after_binding, &extended_context)
//           // }
//     }
// }
