use std::sync::Arc;
use crate::core::{QHsmHelper, ThreadedCodeExecutor};
use std::any::Any;

// pub fn switch_entry(void* data) {
// }

// pub fn switch_init(void* data) {
// }

pub fn off_entry(_data: Option<&Box<dyn Any>>) {
    println!("OFF");
}

pub fn off_reset(_data: Option<&Box<dyn Any>>) {
    println!("@RESET");
}

// pub fn off_exit(void* data) {
// }

pub fn off_turn(_data: Option<&Box<dyn Any>>) {
    println!("OFF: TURN");
}

pub fn on_entry(_data: Option<&Box<dyn Any>>) {
    println!("ON");
}

// pub fn on_exit(void* data) {
// }

pub fn on_turn(_data: Option<&Box<dyn Any>>) {
    println!("ON:  TURN");
}

pub fn create_helper(helper: &Arc<QHsmHelper>) {
    let switch_init_functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>> = vec![
        // Box::new(switch_entry),
        // Box::new(switch_init),
        Box::new(off_entry)
        ];
    helper.insert("switch".to_string(), "init".to_string(), Arc::new(ThreadedCodeExecutor::new(switch_init_functions, "off".to_string())));

    let off_reset_functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>> = vec![
        Box::new(off_reset),
        // Box::new(off_exit),
        // Box::new(switch_init),
        Box::new(off_entry)
        ];
    helper.insert("off".to_string(), "RESET".to_string(), Arc::new(ThreadedCodeExecutor::new(off_reset_functions, "off".to_string())));

    let off_turn_functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>> = vec![
        Box::new(off_turn), 
        Box::new(on_entry)
        ];
    helper.insert("off".to_string(), "TURN".to_string(), Arc::new(ThreadedCodeExecutor::new(off_turn_functions, "on".to_string())));

    let on_reset_functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>> = vec![
        Box::new(off_reset),
        // Box::new(on_exit),
        // Box::new(off_exit),
        // Box::new(switch_init),
        Box::new(off_entry)
        ];
    helper.insert("on".to_string(), "RESET".to_string(), Arc::new(ThreadedCodeExecutor::new(on_reset_functions, "off".to_string())));

    let on_turn_functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>> = vec![
        Box::new(on_turn),
        // Box::new(on_exit),
        // Box::new(off_exit),
        // Box::new(switch_init),
        Box::new(off_entry)
        ];
    helper.insert("on".to_string(), "TURN".to_string(), Arc::new(ThreadedCodeExecutor::new(on_turn_functions, "off".to_string())));
}
