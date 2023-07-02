use std::rc::Rc;

use crate::parser::parse_node::*;
use crate::{evaluate_input, lex, message::*, setup_context};

use super::{context::*, data::*, function::*, eval_helpers_tco::*};


// push results straight to res_q
    // because function call is resolved recursively
    // later: this can change

// FnDef: returns ExpressionResult
// if: returns DeferredExpr
// let: returns ExpressionResult (or Deferred)
    // evaluate before returning
    // alt: take the last expr + ctx out and put on stack as deferred

// #[derive(Display)]
// pub enum Expression {
//     Deferred(DeferredExpression),
//     Result(EvaluatedExpression)
// }

pub struct FunctionCall {
    pub func:Rc<dyn Function>,
    pub ast:Rc<ASTNode>,
    pub parent:Option<Rc<ASTNode>>
}

// an expression on the call stack
    // separate struct because parent pointer is always set after returning from another function
    // so inside a sub-function we can just return part of it and centralise setting of the parent ptr
pub struct StackExpression {
    pub expr:DeferredExpression,
    pub parent:Option<Rc<ASTNode>>
}

// body: used for eval, parent: used for checking
pub struct DeferredExpression {
    pub ctx:EvalContext,
    pub body:Rc<ASTNode>,
}

// this will get transferred to the result queue
// pub struct EvaluatedExpression {
//     data:DataValue,
// }

pub struct ExpressionResult {
    pub data:DataValue,
    pub parent:Option<Rc<ASTNode>>
}

pub(crate) fn evaluate_outer(ctx: EvalContext, node: Rc<ASTNode>, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());

    let deferred=DeferredExpression {
        ctx:ctx.clone(),
        body:Rc::clone(&node)
    };

    let stack_expr=StackExpression {
        expr:deferred,
        parent:node.parent.clone()
    };

    evaluate_tco(stack_expr, outer_call)
}

// why does this take EvalContext without ref:
    // because of issue where when returning from UserFunction execute we need a new owned context
    // can't return &Eval since it's created inside the fn body
// Good thing is EvalContext.clone() is cheap because of Rc::clone
use std::collections::VecDeque;


fn resolve(call_stack:&mut VecDeque<StackExpression>, fn_stack:&mut VecDeque<FunctionCall>,results:&mut VecDeque<ExpressionResult>)->Result<()> {
    let expression=call_stack.pop_back().unwrap();
    let parent=&expression.parent;
    let expr=&expression.expr;

    let ctx=&expr.ctx;
    let body=&expr.body;
    
    let mut result=ExpressionResult {
        data:Num(-1),
        parent:parent.clone()
    };

    match &body.value {
        Number(n) => {
            result.data=Num(*n);
            results.push_back(result);
        },
        Boolean(b) => {
            result.data=Bool(*b);
            results.push_back(result);
        },
        Symbol(sym) => {
            // functon
            let read=ctx.read();
            let data=read.get_data_value(sym);

            match data {
                Some(value) => {
                    result.data=value.clone();
                    results.push_back(result);
                },
                None => {
                   let err=format!("Unrecognised symbol: {}", sym);
                   return err!(err);
                }
            }
        },
        IfNode(children) => {
            let res=evaluate_if(ctx, children)?;
            let stack_expr=StackExpression {
                expr:res,
                parent:parent.clone()
            };
            call_stack.push_back(stack_expr);
        }
        _ => {
            todo!();
        }
    }

    Ok(())
}

fn evaluate_tco(expression:StackExpression, outer_call: bool) -> Result<DataValue> {
    // try to match terminals
    // println!("Node type: {}, Expr: {}", node.get_type(), node.to_string_with_parent());
    let mut call_stack: VecDeque<StackExpression> = VecDeque::new();
    let mut fn_stack: VecDeque<FunctionCall> = VecDeque::new();
    let mut results_queue: VecDeque<ExpressionResult> = VecDeque::new();
    let expr_string=&expression.expr.body.to_string();

    call_stack.push_back(expression);

    // what to do with expression on call_st when valid
        // valid: fn_st empty and call_st not empty OR fn_st[-1].ast==call_st[-1].parent
    

    // call_st only: unroll the expr
    // fn_stack only: check res_q
    // both: check fn_st[-1].ast vs call_st[-1].parent
    while !call_stack.is_empty() || !fn_stack.is_empty() {
        let call_has=!call_stack.is_empty();
        let fn_has=!fn_stack.is_empty();

        // both
        if call_has && fn_has {

        }

        // call only
        else if call_has && !fn_has {
            resolve(&mut call_stack, &mut fn_stack, &mut results_queue)?;
        }
        
        // fn only - fn.execute
        else {

        }
    }

    match results_queue.into_iter().last() {
        Some(res) => {
            Ok(res.data)
        },
        None => {
            let msg=format!("Could not evaluate expression: {}", expr_string);
            return err!(msg);
        }
    }
}

