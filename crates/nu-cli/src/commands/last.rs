use crate::commands::WholeStreamCommand;
use crate::context::CommandRegistry;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value};
use nu_source::Tagged;

pub struct Last;

#[derive(Deserialize)]
pub struct LastArgs {
    rows: Option<Tagged<u64>>,
}

#[async_trait]
impl WholeStreamCommand for Last {
    fn name(&self) -> &str {
        "last"
    }

    fn signature(&self) -> Signature {
        Signature::build("last").optional(
            "rows",
            SyntaxShape::Number,
            "starting from the back, the number of rows to return",
        )
    }

    fn usage(&self) -> &str {
        "Show only the last number of rows."
    }

    async fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        last(args, registry).await
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Get the last row",
                example: "echo [1 2 3] | last",
                result: Some(vec![Value::from(UntaggedValue::from(BigInt::from(3)))]),
            },
            Example {
                description: "Get the last three rows",
                example: "echo [1 2 3 4 5] | last 3",
                result: Some(vec![
                    UntaggedValue::int(3).into(),
                    UntaggedValue::int(4).into(),
                    UntaggedValue::int(5).into(),
                ]),
            },
        ]
    }
}

async fn last(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let registry = registry.clone();
    let (LastArgs { rows }, input) = args.process(&registry).await?;
    let v: Vec<_> = input.into_vec().await;

    let rows_desired = if let Some(quantity) = rows {
        *quantity
    } else {
        1
    };

    let mut values_vec_deque = VecDeque::new();

    let count = rows_desired as usize;

    if count < v.len() {
        let k = v.len() - count;

        for x in v[k..].iter() {
            values_vec_deque.push_back(ReturnSuccess::value(x.clone()));
        }
    }

    Ok(futures::stream::iter(values_vec_deque).to_output_stream())
}

#[cfg(test)]
mod tests {
    use super::Last;

    #[test]
    fn examples_work_as_expected() {
        use crate::examples::test as test_examples;

        test_examples(Last {})
    }
}
