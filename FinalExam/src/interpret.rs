use std::collections::HashMap;
use crate::ast::{Program, Statement, Expr, Identifier};

pub type Env = HashMap<Identifier, f64>;

pub struct TurtleState {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub pen_down: bool,
}

impl TurtleState {
    pub fn new(start_x: f64, start_y: f64) -> Self {
        TurtleState {
            x: start_x,
            y: start_y,
            angle: 0.0,
            pen_down: false,
        }
    }
}

pub fn evaluate_expression(expression: &Expr, env: &Env) -> Result<f64, String> {
    match expression {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(ident) => {
            env.get(ident)
                .copied()
                .ok_or_else(|| format!("Undefined variable: '{}'", ident.0))
        }
    }
}

pub fn evaluate_statement(
    stat: &Statement,
    environment: &mut Env,
    turtle: &mut TurtleState,
    draw_line: &mut impl FnMut(f64, f64, f64, f64),
) -> Result<(), String> {
    match stat {
        Statement::Forward(expr) => {
            let dist = evaluate_expression(expr, environment)?;
            let rad = turtle.angle.to_radians();
            let new_x = turtle.x + dist * rad.sin();
            let new_y = turtle.y - dist * rad.cos();

            if turtle.pen_down {
                draw_line(turtle.x, turtle.y, new_x, new_y);
            }

            turtle.x = new_x;
            turtle.y = new_y;
        }

        Statement::Turn(expr) => {
            let degrees = evaluate_expression(expr, environment)?;
            turtle.angle += degrees;
        }

        Statement::Pen(expr) => {
            let val = evaluate_expression(expr, environment)?;
            turtle.pen_down = val != 0.0;
        }

        Statement::Set { name, value } => {
            let val = evaluate_expression(value, environment)?;
            environment.insert(name.clone(), val);
        }

        Statement::Dotimes { count, body } => {
            let n = evaluate_expression(count, environment)? as usize;
            for _ in 0..n {
                for stat in body {
                    evaluate_statement(stat, environment, turtle, draw_line)?;
                }
            }
        }
    }

    Ok(())
}

pub fn interpret(
    program: &Program,
    turtle: &mut TurtleState,
    draw_line: &mut impl FnMut(f64, f64, f64, f64),
) -> Result<(), String> {
    let mut env: Env = HashMap::new();
    for stat in &program.statements {
        evaluate_statement(stat, &mut env, turtle, draw_line)?;
    }
    Ok(())
}