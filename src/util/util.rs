use gloo_net::http::{Request, Response};
use gloo_storage::{LocalStorage, Storage};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Display;
use thiserror::Error;
use web_sys::console::error_1 as console_err1;
use web_sys::MouseEvent;
use yew::Callback;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("failed setting up request to url {0}: {1:?}")]
    SetupBuildError(String, gloo_net::Error),
    #[error("failed making request to url {0}: {1:?}")]
    RequestSendError(String, gloo_net::Error),
    #[error("invalid status code {0} from url {1}: {2}")]
    InvalidStatusCodeWithMessage(u16, String, String),
    #[error("invalid status code {0} from url {1}")]
    InvalidStatusCodeWithoutMessage(u16, String),
    #[error("failed to prase json from url {0}: {1:?}")]
    ParseError(String, gloo_net::Error),
}

pub struct RequestUtil {
    url: String,
    token: String,
}

impl RequestUtil {
    pub fn new(url: String) -> Self {
        let local_storage = LocalStorage::raw();
        let token = match local_storage.get("token") {
            Ok(value_opt) => match value_opt {
                Some(value) => value,
                None => String::new(),
            },
            Err(_s) => String::from("err"),
        };

        Self { url, token }
    }

    async fn send(
        &self,
        request: Result<Request, gloo_net::Error>,
    ) -> Result<ResponseUtil, UtilError> {
        let request = match request {
            Ok(request) => request,
            Err(e) => {
                return Err(UtilError::SetupBuildError(self.url.clone(), e));
            }
        };

        let response = match request.send().await {
            Ok(response) => response,
            Err(e) => return Err(UtilError::RequestSendError(self.url.clone(), e)),
        };

        Ok(ResponseUtil::new(response))
    }

    pub async fn _post(&self, data: Value) -> Result<ResponseUtil, UtilError> {
        self.send(
            Request::post(&self.url)
                .header("token", &self.token)
                .json(&data),
        )
        .await
    }

    pub async fn get(&self) -> Result<ResponseUtil, UtilError> {
        self.send(Request::get(&self.url).header("token", &self.token).build())
            .await
    }

    pub async fn _put(&self, data: Value) -> Result<ResponseUtil, UtilError> {
        self.send(
            Request::put(&self.url)
                .header("token", &self.token)
                .json(&data),
        )
        .await
    }

    pub async fn _delete(&self) -> Result<ResponseUtil, UtilError> {
        self.send(
            Request::delete(&self.url)
                .header("token", &self.token)
                .build(),
        )
        .await
    }
}

pub struct ResponseUtil {
    response: Response,
}

impl ResponseUtil {
    pub fn new(response: Response) -> Self {
        Self { response }
    }

    async fn bad_status_code_message(&self, code: u16) -> UtilError {
        match self.response.text().await {
            Ok(msg) => UtilError::InvalidStatusCodeWithMessage(code, self.response.url(), msg),
            Err(_) => UtilError::InvalidStatusCodeWithoutMessage(code, self.response.url()),
        }
    }

    pub async fn _no_content(&self, code: u16) -> Result<(), UtilError> {
        if self.response.status() != code {
            Err(self.bad_status_code_message(code).await)
        } else {
            Ok(())
        }
    }

    pub async fn json<T>(&self, code: Vec<u16>) -> Result<T, UtilError>
    where
        T: DeserializeOwned,
    {
        let status_code = self.response.status();
        if !code.contains(&status_code) {
            Err(self
                .bad_status_code_message(*code.first().unwrap_or(&0))
                .await)
        } else {
            match self.response.json::<T>().await {
                Ok(v) => Ok(v),
                Err(e) => Err(UtilError::ParseError(self.response.url(), e)),
            }
        }
    }
}

pub fn onclick_event_manager<T: 'static>(
    handlers: Vec<Callback<T>>,
    item: T,
) -> Callback<MouseEvent>
where
    T: Clone,
{
    let handlers = handlers.clone();
    let item = item.clone();
    Callback::from(move |_e: MouseEvent| {
        let handlers = handlers.clone();
        handlers.into_iter().for_each(|h| h.emit(item.clone()))
    })
}

pub fn console_err<T: 'static>(e: &T)
where
    T: Display,
{
    console_err1(&format! {"{}", e}.into())
}
