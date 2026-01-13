// use std::collections::HashMap;

// use crate::ast::Expression;

// #[derive(Debug)]
// pub enum EvaluationError {
//     UnboundedVariable(String),
//     NotAFunction,
//     ConditionalNotBool,
// }

// #[derive(Debug, Clone)]
// pub enum Value {
//     Bool(bool),
//     Closure {
//         parameter: String,
//         body: Box<Expression>,
//         environment: Environment,
//     },
// }

// pub type Environment = HashMap<String, Value>;

// pub fn evaluate(
//     expression: &Expression,
//     environment: &Environment,
// ) -> Result<Value, EvaluationError> {
//     match expression {
//         Expression::Variable(name) => environment
//             .get(name)
//             .cloned()
//             .ok_or_else(|| EvaluationError::UnboundedVariable(name.clone())),
//         Expression::Lambda {
//             parameter, body, ..
//         } => Ok(Value::Closure {
//             parameter: parameter.clone(),
//             body: (*body).clone(),
//             environment: environment.clone(),
//         }),
//         Expression::Application { function, argument } => {
//             let function_value = evaluate(function, environment)?;
//             let argument_value = evaluate(argument, environment)?;

//             match function_value {
//                 Value::Closure {
//                     parameter,
//                     body,
//                     environment: mut closure_environment,
//                 } => {
//                     closure_environment.insert(parameter, argument_value); // TODO: DE BRUIJN
//                     evaluate(&body, &closure_environment)
//                 }
//                 _ => Err(EvaluationError::NotAFunction), // unreachable after type checking
//             }
//         }
//         // Expression::Bool(b) => Ok(Value::Bool(*b)),
//         // Expression::If {
//         //     condition,
//         //     then_branch,
//         //     else_branch,
//         // } => {
//         //     let conditional_value = evaluate(condition, environment)?;
//         //     match conditional_value {
//         //         Value::Bool(true) => evaluate(then_branch, environment),
//         //         Value::Bool(false) => evaluate(else_branch, environment),
//         //         _ => Err(EvaluationError::ConditionalNotBool),
//         //     }
//         // }
//         // Expression::LetBinding {
//         //     variable_name,
//         //     bound_to,
//         //     expression_after_binding,
//         // } => {
//         //     let bound_to_value = evaluate(bound_to, environment)?;
//         //     let mut new_environment = environment.clone();
//         //     new_environment.insert(variable_name.clone(), bound_to_value); // TODO: DE BRUIJN
//         //     evaluate(expression_after_binding, &new_environment)
//         // }
//     }
// }
