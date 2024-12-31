use std::sync::Arc;
use crate::core::{QHsmHelper, Runner, post};
use crate::switch_reset_helper::create_helper;

pub fn test_switch() {
    let helper = Arc::new(QHsmHelper::new("switch".to_string()));
    let runner = Runner::new(helper.clone());
    create_helper(&helper);

    post(&runner, "init".to_string(), None);
    post(&runner, "TURN".to_string(), None);
    post(&runner, "RESET".to_string(), None);
    post(&runner, "TURN".to_string(), None);
    post(&runner, "TURN".to_string(), None);
    post(&runner, "RESET".to_string(), None);
}
