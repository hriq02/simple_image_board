use std::fmt::Display;




pub enum ServerError{
    DatabaseError(String),
    RequestError(String),
    IoError(String),
    OtherError(String),
    #[allow(dead_code)]
    GraphQLError(String),
}


impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::DatabaseError(err) => {write!(f, "Database Error: {}", err)}
            ServerError::IoError(err) => {write!(f, "IO Error: {}", err)}
            ServerError::OtherError(err) => {write!(f, "Other Error: {}", err)}
            ServerError::RequestError(err) => {write!(f, "Request Error: {}", err)}
            ServerError::GraphQLError(err) => {write!(f, "GraphQL Error: {}", err)}
        }
    }
}

impl From<async_graphql::Error> for ServerError {
    fn from(err: async_graphql::Error) -> Self {
        ServerError::RequestError(format!("{:?}", err))
    }
}


impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> Self {
        ServerError::DatabaseError(err.to_string())
    }
}


impl From <std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IoError(err.to_string())
    }
}

impl From <String> for ServerError {
    fn from(err: String) -> Self {
        ServerError::OtherError(err)
    }
}


#[allow(dead_code)]
trait IntoGqlError<T> {
    fn into_gql(self, context: &str) -> Result<T, ServerError>;
}

impl<T, E: Display> IntoGqlError<T> for Result<T, E> {
    fn into_gql(self, context: &str) -> Result<T, ServerError> {
        self.map_err(|e| ServerError::GraphQLError(format!("{}: {}", context, e)))
    }
}