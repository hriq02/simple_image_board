use std::{fs::OpenOptions, io::BufWriter};
use std::io::Write;
use async_graphql::parser::types::ExecutableDocument;
use async_graphql::Variables;
use async_graphql::{extensions::Extension, Response, ServerError};
use std::sync::Arc;
use async_graphql::extensions::{ ExtensionContext, ExtensionFactory, NextExecute, NextParseQuery};


const GREEN : &str = "\x1b[32m";
const RED : &str = "\x1b[31m";
const YELLOW : &str = "\x1b[33m";
const NORMAL : &str = "\x1b[0m";


pub enum LogLevel {
    Info,
    Warn,
    Error
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
} 

pub fn log_err<E: std::fmt::Display>(message: &str, e: E, level: LogLevel) {
    log(&format!("{}: {}", message, e), level);
}

pub fn log(message : &str, level : LogLevel) {
    match level {
        LogLevel::Error => {
            let msg = log_formated(message, level);
            println!("{}", msg);
            match write_to_file(&msg){
                Ok(_) => (),
                Err(e) => println!("Error writing to file: {}", e)
            }
        },
        _ => {
            let msg = log_formated(message, level);
            println!("{}", msg);
        }

    }
}

fn write_to_file(message : &str) -> std::io::Result<()> {
    writeln!(
        BufWriter::new(
            OpenOptions::new()
            .append(true)
            .create(true)
            .open(chrono::Local::now().format("%Y-%m-%d.log").to_string())?
        )
        
    , "{}", message)?;
    
    Ok(())
}


fn log_formated(message : &str, level : LogLevel) -> String{
    let hour = chrono::Local::now().format("%H:%M:%S").to_string();
    let color = match level {
        LogLevel::Info => GREEN,
        LogLevel::Warn => YELLOW,
        LogLevel::Error => RED
    };
    format!("{}|{}{}{}|{}", hour,color,&level.to_string(),NORMAL,&message)
}




pub struct LogExtension;

impl ExtensionFactory for LogExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(LogExtensionImpl)
    }
}

struct LogExtensionImpl;

#[async_trait::async_trait]
impl Extension for LogExtensionImpl {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> Result<ExecutableDocument, ServerError> {
        crate::logger::log(
            &format!("GraphQL Query: {}", query),
            crate::logger::LogLevel::Info,
        );
        next.run(ctx, query,variables).await
    }

    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let resp = next.run(ctx, operation_name).await;

        if !resp.errors.is_empty() {
            for err in &resp.errors {
                crate::logger::log_err(
                    "GraphQL Error",
                    err.clone(),
                    crate::logger::LogLevel::Error,
                );
            }
        }

        resp
    }
}