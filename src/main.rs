use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Clone)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(e: CustomEvent, _ctx: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        bail!("Empty first name");
    }
    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_handler_success_branch() {
        let e = super::CustomEvent { first_name: String::from("Fred"), last_name: String::from("Johnson") };
        let r = super::my_handler(e, context_for_tests());
        match r {
            Result::Ok(_) => (),
            Result::Err(_) => panic!("expected no error")
        }
    }

    #[test]
    fn my_handler_error_branch() {
        let e = super::CustomEvent { first_name: String::from(""), last_name: String::from("") };
        let r = super::my_handler(e, context_for_tests());
        match r {
            Result::Ok(_) => panic!("expected an error"),
            Result::Err(_) => ()
        }
    }

    fn context_for_tests() -> lambda_runtime::Context {
        return lambda_runtime::Context {
            memory_limit_in_mb: 20,
            function_name: String::from("example_function"),
            function_version: "".to_string(),
            invoked_function_arn: String::from("arn::fake-arn"),
            aws_request_id: "".to_string(),
            xray_trace_id: Some("".to_string()),
            log_stream_name: "".to_string(),
            log_group_name: "".to_string(),
            client_context: None,
            identity: None,
            deadline: 0,
        };
    }
}