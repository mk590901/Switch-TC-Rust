use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::Any;

// Define a structure for the event wrapper
pub struct EventWrapper {
    pub data: Option<Box<dyn Any>>,
    pub event: String,
}

impl EventWrapper {
    pub fn new(event: String, data: Option<Box<dyn Any>>) -> Self {
        EventWrapper { event, data }
    }
}

// Define a structure for the threaded code executor
pub struct ThreadedCodeExecutor {
    pub functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>>,
    pub target_state: String,
}

impl ThreadedCodeExecutor {
    pub fn new(functions: Vec<Box<dyn Fn(Option<&Box<dyn Any>>)>>, target_state: String) -> Self {
        ThreadedCodeExecutor { functions, target_state }
    }

    pub fn execute_sync(&self, data: Option<&Box<dyn Any>>) {
        for func in &self.functions {
            func(data);
        }
    }
}

// Define a structure for the QHsmHelper
pub struct QHsmHelper {
    pub state: Arc<Mutex<String>>,
    pub container: Arc<Mutex<HashMap<String, Arc<ThreadedCodeExecutor>>>>,
}

impl QHsmHelper {
    pub fn new(initial_state: String) -> Self {
        QHsmHelper {
            state: Arc::new(Mutex::new(initial_state)),
            container: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, state: String, event: String, executor: Arc<ThreadedCodeExecutor>) {
        let key = create_key(&state, &event);
        self.container.lock().unwrap().insert(key, executor);
    }

    pub fn set_state(&self, state: String) {
        *self.state.lock().unwrap() = state;
    }

    pub fn get_state(&self) -> String {
        self.state.lock().unwrap().clone()
    }

    pub fn executor(&self, event: &str) -> Option<Arc<ThreadedCodeExecutor>> {
        let key = create_key(&self.get_state(), event);
        self.container.lock().unwrap().get(&key).cloned()
    }
}

// Define a structure for the runner
pub struct Runner {
    pub events_queue: Arc<Mutex<Vec<EventWrapper>>>,
    pub helper: Arc<QHsmHelper>,
}

impl Runner {
    pub fn new(helper: Arc<QHsmHelper>) -> Self {
        Runner {
            events_queue: Arc::new(Mutex::new(Vec::new())),
            helper,
        }
    }
}

// Function to post an event
pub fn post(runner: &Runner, event: String, data: Option<Box<dyn Any>>) {
    runner.events_queue.lock().unwrap().push(EventWrapper::new(event, data));
    while let Some(event_wrapper) = runner.events_queue.lock().unwrap().pop() {
        if let Some(executor) = runner.helper.executor(event_wrapper.event.as_str()) {
            runner.helper.set_state(executor.target_state.clone());
            executor.execute_sync(event_wrapper.data.as_ref());
        }
    }
}

// Function to create a key from state and event
pub fn create_key(state: &str, event: &str) -> String {
    format!("{}.{}", state, event)
}
