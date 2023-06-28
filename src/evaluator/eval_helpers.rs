use super::{context::*, data::*, evaluator::evaluate};
use crate::constants::DONT_ADD;
use crate::message::*;
use crate::parser::node::*;

// evaluated args
pub fn get_eval_args_from_nodes<'a>(
    iter: impl Iterator<Item = Result<DataValue>> + Clone,
) -> Result<Vec<Arg<'a>>> {
    // let eval_rest=nodes.iter().map(|node| evaluate(ctx, node));
    let res_iter: Result<Vec<DataValue>> = iter.clone().into_iter().collect();
    let results = res_iter.map(|v| {
        let args: Vec<Arg> = v.into_iter().map(|x| Arg::Evaluated(x)).collect();
        return args;
    })?;
    Ok(results)
}

pub fn is_valid_identifier(s:String)->bool {
    if DONT_ADD.contains(&s.as_str()) {
        return false;
    }

    
    true
}

// evaluate first child. if len==1, return
// elif first child is FnVar | FnDef => apply to arguments
// else: evaluate nodes in order, return result from last
pub fn evaluate_expression(ctx: &Context, children: &Vec<ASTNode>) -> Result<DataValue> {
    if children.is_empty() {
        return Err(Ex::new("Received empty expression."));
    }

    let first_child = children.first().unwrap();
    let res = evaluate(ctx, first_child)?;

    if children.len() == 1 {
        return Ok(res);
    }

    let mut rest = children.iter();
    rest.next();

    let eval_rest = rest.clone().map(|node| evaluate(ctx, node));

    // is function: check ArgType, gets arg, eval.
    match res.expect_function().ok() {
        Some(func) => {
            if func.get_arg_type() == ArgType::Evaluated {
                let results = get_eval_args_from_nodes(eval_rest.clone())?;
                func.execute(results, ctx)

                // let strings:Vec<String>=results.into_iter().map(|x| x.to_string()).collect();
                // dbg!(strings);
            } else {
                // just ast nodes
                let args: Vec<Arg> = children.into_iter().map(|x| Unevaluated(x)).collect();
                func.execute(args, ctx)
            }
        }
        // not a function: evaluate in order and return last
        None => {
            let res_iter: Result<Vec<DataValue>> = eval_rest.clone().into_iter().collect();
            res_iter?
                .into_iter()
                .last()
                .ok_or(Ex::new("Couldn't evaluate expression."))
        }
    }
}

pub fn evaluate_list(_ctx: &Context, children: &Vec<ASTNode>) -> Result<DataValue> {
    println!("list eval");
    dbg!(children);
    Ok(Default)
}

pub fn evaluate_if(ctx: &Context, cond: &ASTNode, e1: &ASTNode, e2: &ASTNode) -> Result<DataValue> {
    // println!("Received if eval: Cond: {} e1: {} e2: {}", cond.to_string(), e1.to_string(), e2.to_string());
    let cond_result = evaluate(ctx, cond)?;

    // add empty list as false later
    let condition = match cond_result {
        Num(num) => num != 0,
        Bool(b) => b,
        _ => true,
    };

    if condition {
        evaluate(ctx, e1)
    } else {
        evaluate(ctx, e2)
    }
}

pub fn evaluate_let(ctx: &Context, expressions: &Vec<ASTNode>) -> Result<DataValue> {
    // println!("Let received eval:", expressions.);
    // expressions.iter().for_each(|n| println!("{}", n.to_string()));
    let new_ctx=ctx.clone();

    Ok(Default)
}
