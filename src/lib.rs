mod app;
mod config;
mod navigation;
mod terminal;
mod tui;

use anyhow::{Context, Result};

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Type, Value};

use app::{Mode, State};
use config::Config;
use terminal::restore as restore_terminal;
use terminal::setup as setup_terminal;

pub struct Explore;

impl Plugin for Explore {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("explore")
            .usage("TODO")
            .input_output_type(Type::Any, Type::Any)
            .plugin_examples(vec![PluginExample {
                example: "open Cargo.toml | explore".into(),
                description: "TODO".into(),
                result: None,
            }])
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "explore" => explore(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn explore(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let config = Config::default();

    let mut terminal = setup_terminal().context("setup failed").unwrap();
    let result = app::run(&mut terminal, input, &config).context("app loop failed");
    restore_terminal(&mut terminal)
        .context("restore terminal failed")
        .unwrap();

    match result {
        Ok(res) => Ok(res),
        Err(err) => Err(LabeledError {
            label: "unexpected error".into(),
            msg: err.to_string(),
            span: Some(call.head),
        }),
    }
}
