use crate::CustomError::CustomError;

pub(crate) struct ExecuteErrorChain {
    error : dyn CustomError,
    previous : Option<dyn CustomError>
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
    ErrorChain(ExecuteErrorChain),
    Result(T)
}

pub(crate) trait IChainable<T, T2>{
    fn ChainWith(&self, next : fn(prev_result : T) -> Result<T2, dyn CustomError>) -> ProcessResult<T2>;
}

impl<T1, T2> IChainable<T1, T2> for Result<T1, dyn CustomError> {
    fn ChainWith(&self, next: fn(prev_result : T1) -> Result<T2, dyn CustomError>) -> ProcessResult<T2> {
        match self {
            Ok(value ) => {
                let res2 = next(value);
                return Process(res2);
            },
            Err(err) =>{
                ProcessResult::ErrorChain(
                    ExecuteErrorChain{
                        error : err,
                        previous : None
                    }
                )
            }
        }
    }
}

pub(crate) fn Process<TResult>(result : Result<TResult, dyn CustomError>) -> ProcessResult<TResult>{

    match result {
        Ok(value ) =>{
            ProcessResult::Result(value)
        },
        Err(err) =>{
            ProcessResult::ErrorChain(
                ExecuteErrorChain{
                    error : err,
                    previous : None
                }
            )
        }
    }
}


