use std::rc::Rc;
use std::fmt::Display;

use crate::constants::NumType;
use crate::parser::node::ASTNode;
use crate::message::*;
use super::function::Function;

pub const NUM:&str="Num";
pub const BOOL:&str="Bool";
pub const FNV:&str="FunctionVariable";

// Number, Boolean, List, String, Lambda, FunctionVariable(Box<dyn Function>)
// when we have an enum that has a reference, then a vector of enums and I clone the vector what happens

// do a getter for each enum type that returns an Option so we can chain map etc

// Function shouldn't get dropped until all refs in context/args are dropped -> use Rc
#[derive(Clone,Display,AsRefStr)]
pub enum DataValue {
    Num(NumType),
    Boolean(bool),
    FunctionVariable(Rc<dyn Function>), // we need to borrow the function from Context when doing this
    Default,
}

impl DataValue  {
    pub fn expect_num(&self) -> Result<NumType> {
        match self {
            Num(num) => Ok(*num),
            _ => {
                let msg=format!("Expected a number but got '{}'", self.to_string());
                Err(Ex::new(msg.as_str()))
            },
        }
    }

    pub fn expect_bool(&self) -> Result<bool> {
        match self {
            Boolean(bool) => Ok(*bool),
            _ => {
                let msg=format!("Expected a boolean but got '{}'", self.to_string());
                Err(Ex::new(msg.as_str()))
            },
        }
    }

    pub fn expect_function(&self) -> Result<&Rc<dyn Function>> {
        match self {
            FunctionVariable(fn_ref) => Ok(fn_ref),
            _ => {
                let msg=format!("Expected a function but got '{}'", self.to_string());
                Err(Ex::new(msg.as_str()))
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Num(n) => n.to_string(),
            Boolean(b) => b.to_string(),
            FunctionVariable(f) => f.to_string(),
            Default => String::from("Default Data Value")
        }
    }
}

// expect_eval: takes one arg -> Option<DataValue> (None if uneval)
    // expect_uneval: ... 
    // consume because we need to unwrap the value inside => Arg is useless after that

// expect_all: (Iterator<Arg>, predicate(Arg)) -> true if all ...


pub enum Arg<'a> {
    Evaluated(DataValue),
    Unevaluated(&'a ASTNode), // node could be part of fn body -> Arg can't own it
}

pub use Arg::*;

impl<'a> Arg<'a>  {
    pub fn expect_eval(self)->Result<DataValue>{
        match self {
            Evaluated(val) => Ok(val),
            Unevaluated(node)=> {
                let msg=format!("Expected evaluated: {}", node.to_string());
                Err(Ex::new(msg.as_str()))
            }
        }
    }

    pub fn expect_uneval(self)->Result<&'a ASTNode> {
        match self {
            Unevaluated(node) => Ok(node),
            Evaluated(val) => {
                let msg=format!("Expected unevaluated node: {}", val.to_string());
                Err(Ex::new(msg.as_str()))
            }
        }
    }

    pub fn expect_all_eval(args:Vec<Arg>)->Result<Vec<DataValue>> {
        let k:Result<Vec<DataValue>>=args.into_iter().map(|a| a.expect_eval()).collect();
        return k;
    }

    pub fn expect_all_uneval(args:Vec<Arg<'a>>)->Result<Vec<&'a ASTNode>> {
        let k:Result<Vec<&'a ASTNode>>=args.into_iter().map(|a| a.expect_uneval()).collect();
        return k;
    }

    pub fn to_string(&self)->String {
        match self {
            Evaluated(val) => val.to_string(),
            Unevaluated(node) => node.to_string(),
        }
    }
}

pub use Arg::*;

impl<'a> Display for Arg<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

#[derive(PartialEq)]
pub enum ArgType {
    Evaluated,
    Unevaluated,
}

pub use Arg::*;
pub use DataValue::*;

#[cfg(test)]
    pub mod tests {
        use super::*;
        use super::DataValue::*;
        use super::super::builtins::Add;
        use std::rc::Rc;

        #[test]
        fn data_test_getters() {
            let d1=Num(20);
            let d2=Boolean(true);
            let add=Add{};
            let d3=FunctionVariable(Rc::new(add));

            dbg!(d3.to_string());

            assert_eq!(d1.expect_num().unwrap(), 20);
            assert!(d2.expect_num().is_err());
            assert!(d3.expect_num().is_err());

            assert!(d1.expect_bool().is_err());
            assert_eq!(d2.expect_bool().unwrap(), true);
            assert!(d3.expect_bool().is_err());

            assert!(d1.expect_function().is_err());
            assert!(d2.expect_function().is_err());
            assert!(d3.expect_function().is_ok());            
        }

        #[test]
        fn data_test_arg_expect() {
            let d=DataValue::Boolean(true);
            let d1=DataValue::Num(20);
            let v1:Vec<Arg>=vec![Evaluated(d),Evaluated(d1)];

            let res=Arg::expect_all_eval(v1);
            assert!(res.is_ok());
            let n=res.unwrap();
        
            assert_eq!(n.get(0).unwrap().to_string(), "true");
        }
    }

