use crate::{eval};
use crate::parser::ast::{Term, NodeType, Value};

pub fn interpreter(baik_script: &str) {
    let baiks = Term::input(baik_script).unwrap();
    for baik in baiks {
        match baik.node_type() {
            NodeType::Boolean => {
                baik.boolean().unwrap().clone().value();
            },
            // NodeType::Declaration => {
            //     let (ident, value) = baik.declaration().unwrap();
            //     let value = match value.clone() {
            //         NodeType::Integer => value.integer().unwrap()
            //     };
            //     println!("Declaration:  {} => {}", ident.value_ref(), value.value_ref());
            // },
            _ => {
                    println!("----");
            }
        }
    }
}