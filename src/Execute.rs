use crate::CustomError::CustomError;

pub(crate) struct ExecuteErrorChain {
    error : dyn CustomError,
    previous : Option<ExecuteError>
}

pub(crate) struct ExecuteError{
    error: String
}

impl CustomError for ExecuteError{
    fn get_error(&self) -> String {
        return self.error.clone()
    }
}

pub(crate) enum ProcessResult<T>{
    ErrorChain: Option<ExecuteErrorChain>,
    result : T
}

pub(crate) fn Process<TResult>(result : Result<TResult, dyn CustomError>) -> ProcessResult<TResult>{

    match result {
        Ok(value ) =>{
            ProcessResult
            {
                result : value,
                error_chain : None
            }
        },
        Err(err) =>{
            ProcessResult{
                result : None
            }
        }
    }
}
