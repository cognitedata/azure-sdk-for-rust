#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://digitaltwins-hostname";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{endpoint}/")]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let context = azure_core::Context::default();
        self.pipeline.send(&context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn delete_jobs_client(&self) -> delete_jobs::Client {
        delete_jobs::Client(self.clone())
    }
    pub fn digital_twin_models_client(&self) -> digital_twin_models::Client {
        digital_twin_models::Client(self.clone())
    }
    pub fn digital_twins_client(&self) -> digital_twins::Client {
        digital_twins::Client(self.clone())
    }
    pub fn event_routes_client(&self) -> event_routes::Client {
        event_routes::Client(self.clone())
    }
    pub fn import_jobs_client(&self) -> import_jobs::Client {
        import_jobs::Client(self.clone())
    }
    pub fn query_client(&self) -> query::Client {
        query::Client(self.clone())
    }
}
pub mod digital_twin_models {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves model metadata and, optionally, model definitions.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The model id is invalid.\n  * LimitExceeded - The maximum number of model ids allowed in 'dependenciesFor' has been reached.\n* 404 Not Found\n  * ModelNotFound - The model was not found."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                traceparent: None,
                tracestate: None,
                dependencies_for: Vec::new(),
                include_model_definition: None,
                max_items_per_page: None,
            }
        }
        #[doc = "Uploads one or more models. When any error occurs, no models are uploaded.\nStatus codes:\n* 201 Created\n* 400 Bad Request\n  * DTDLParserError - The models provided are not valid DTDL.\n  * InvalidArgument - The model id is invalid.\n  * LimitExceeded - The maximum number of model ids allowed in 'dependenciesFor' has been reached.\n  * ModelVersionNotSupported - The version of DTDL used is not supported.\n* 409 Conflict\n  * ModelAlreadyExists - The model provided already exists."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `models_`: An array of models to add."]
        pub fn add(&self, models_: Vec<serde_json::Value>) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                models_,
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Retrieves model metadata and optionally the model definition.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The model id is invalid.\n  * MissingArgument - The model id was not provided.\n* 404 Not Found\n  * ModelNotFound - The model was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the model. The id is globally unique and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
                include_model_definition: None,
            }
        }
        #[doc = "Updates the metadata for a model.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The model id is invalid.\n  * JsonPatchInvalid - The JSON Patch provided is invalid.\n  * MissingArgument - The model id was not provided.\n* 404 Not Found\n  * ModelNotFound - The model was not found.\n* 409 Conflict\n  * ModelReferencesNotDecommissioned - The model refers to models that are not decommissioned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the model. The id is globally unique and case sensitive."]
        #[doc = "* `update_model`: An update specification described by JSON Patch. Only the decommissioned property can be replaced."]
        pub fn update(&self, id: impl Into<String>, update_model: Vec<serde_json::Value>) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                update_model,
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Deletes a model. A model can only be deleted if no other models reference it.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The model id is invalid.\n  * MissingArgument - The model id was not provided.\n* 404 Not Found\n  * ModelNotFound - The model was not found.\n* 409 Conflict\n  * ModelReferencesNotDeleted - The model refers to models that are not deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the model. The id is globally unique and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PagedDigitalTwinsModelDataCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PagedDigitalTwinsModelDataCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) dependencies_for: Vec<String>,
            pub(crate) include_model_definition: Option<bool>,
            pub(crate) max_items_per_page: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "If specified, only return the set of the specified models along with their dependencies. If omitted, all models are retrieved."]
            pub fn dependencies_for(mut self, dependencies_for: Vec<String>) -> Self {
                self.dependencies_for = dependencies_for;
                self
            }
            #[doc = "When true the model definition will be returned as part of the result."]
            pub fn include_model_definition(mut self, include_model_definition: bool) -> Self {
                self.include_model_definition = Some(include_model_definition);
                self
            }
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested number."]
            pub fn max_items_per_page(mut self, max_items_per_page: i64) -> Self {
                self.max_items_per_page = Some(max_items_per_page);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PagedDigitalTwinsModelDataCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(traceparent) = &this.traceparent {
                                    req.insert_header("traceparent", traceparent);
                                }
                                if let Some(tracestate) = &this.tracestate {
                                    req.insert_header("tracestate", tracestate);
                                }
                                let dependencies_for = &this.dependencies_for;
                                for value in &this.dependencies_for {
                                    req.url_mut().query_pairs_mut().append_pair("dependenciesFor", &value.to_string());
                                }
                                if let Some(include_model_definition) = &this.include_model_definition {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeModelDefinition", &include_model_definition.to_string());
                                }
                                if let Some(max_items_per_page) = &this.max_items_per_page {
                                    req.insert_header("max-items-per-page", &max_items_per_page.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/models", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod add {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::NonPagedDigitalTwinsModelDataCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::NonPagedDigitalTwinsModelDataCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) models_: Vec<serde_json::Value>,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.models_)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/models", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::NonPagedDigitalTwinsModelDataCollection>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::NonPagedDigitalTwinsModelDataCollection>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DigitalTwinsModelData> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DigitalTwinsModelData = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) include_model_definition: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "When true the model definition will be returned as part of the result."]
            pub fn include_model_definition(mut self, include_model_definition: bool) -> Self {
                self.include_model_definition = Some(include_model_definition);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        if let Some(include_model_definition) = &this.include_model_definition {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeModelDefinition", &include_model_definition.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/models/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DigitalTwinsModelData>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DigitalTwinsModelData>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) update_model: Vec<serde_json::Value>,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.update_model)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/models/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/models/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
}
pub mod query {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Executes a query that allows traversing relationships and filtering by property values.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * BadRequest - The continuation token is invalid.\n  * SqlQueryError - The query contains some errors.\n  * TimeoutError - The query execution timed out after 60 seconds. Try simplifying the query or adding conditions to reduce the result size.\n * 429 Too Many Requests\n  * QuotaReachedError - The maximum query rate limit has been reached."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `query_specification`: The query specification to execute."]
        pub fn query_twins(&self, query_specification: impl Into<models::QuerySpecification>) -> query_twins::RequestBuilder {
            query_twins::RequestBuilder {
                client: self.0.clone(),
                query_specification: query_specification.into(),
                traceparent: None,
                tracestate: None,
                max_items_per_page: None,
            }
        }
    }
    pub mod query_twins {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::QueryResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::QueryResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The query charge."]
            pub fn query_charge(&self) -> azure_core::Result<f32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("query-charge"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) query_specification: models::QuerySpecification,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) max_items_per_page: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested number."]
            pub fn max_items_per_page(mut self, max_items_per_page: i64) -> Self {
                self.max_items_per_page = Some(max_items_per_page);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.query_specification)?;
                        if let Some(max_items_per_page) = &this.max_items_per_page {
                            req.insert_header("max-items-per-page", &max_items_per_page.to_string());
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/query", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::QueryResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::QueryResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod digital_twins {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves a digital twin.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Adds or replaces a digital twin.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or payload is invalid.\n  * ModelDecommissioned - The model for the digital twin is decommissioned.\n  * TwinLimitReached - The maximum number of digital twins allowed has been reached.\n  * ValidationFailed - The digital twin payload is not valid.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `twin`: The digital twin instance being added. If provided, the $dtId property is ignored."]
        pub fn add(&self, id: impl Into<String>, twin: impl Into<serde_json::Value>) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                twin: twin.into(),
                traceparent: None,
                tracestate: None,
                if_none_match: None,
            }
        }
        #[doc = "Updates a digital twin.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or payload is invalid.\n  * JsonPatchInvalid - The JSON Patch provided is invalid.\n  * ValidationFailed - Applying the patch results in an invalid digital twin.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `patch_document`: An update specification described by JSON Patch. Updates to property values and $model elements may happen in the same request. Operations are limited to add, replace and remove."]
        pub fn update(&self, id: impl Into<String>, patch_document: Vec<serde_json::Value>) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                patch_document,
                traceparent: None,
                tracestate: None,
                if_match: None,
            }
        }
        #[doc = "Deletes a digital twin. All relationships referencing the digital twin must already be deleted.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id is invalid.\n  * RelationshipsNotDeleted - The digital twin contains relationships.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
                if_match: None,
            }
        }
        #[doc = "Retrieves a relationship between two digital twins.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or relationship id is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n  * RelationshipNotFound - The relationship was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        pub fn get_relationship_by_id(
            &self,
            id: impl Into<String>,
            relationship_id: impl Into<String>,
        ) -> get_relationship_by_id::RequestBuilder {
            get_relationship_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Adds a relationship between two digital twins.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id, relationship id, or payload is invalid.\n  * InvalidRelationship - The relationship is invalid.\n  * OperationNotAllowed - The relationship cannot connect to the same digital twin.\n  * ValidationFailed - The relationship content is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n  * TargetTwinNotFound - The digital twin target of the relationship was not found.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        #[doc = "* `relationship`: The data for the relationship."]
        pub fn add_relationship(
            &self,
            id: impl Into<String>,
            relationship_id: impl Into<String>,
            relationship: impl Into<serde_json::Value>,
        ) -> add_relationship::RequestBuilder {
            add_relationship::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                relationship: relationship.into(),
                traceparent: None,
                tracestate: None,
                if_none_match: None,
            }
        }
        #[doc = "Updates the properties on a relationship between two digital twins.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or relationship id is invalid.\n  * InvalidRelationship - The relationship is invalid.\n  * JsonPatchInvalid - The JSON Patch provided is invalid.\n  * ValidationFailed - The relationship content is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n  * RelationshipNotFound - The relationship was not found.\n* 409 Conflict\n  * RelationshipAlreadyExists - The relationship already exists.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        #[doc = "* `patch_document`: JSON Patch description of the update to the relationship properties."]
        pub fn update_relationship(
            &self,
            id: impl Into<String>,
            relationship_id: impl Into<String>,
            patch_document: Vec<serde_json::Value>,
        ) -> update_relationship::RequestBuilder {
            update_relationship::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                patch_document,
                traceparent: None,
                tracestate: None,
                if_match: None,
            }
        }
        #[doc = "Deletes a relationship between two digital twins.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or relationship id is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n  * RelationshipNotFound - The relationship was not found.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        pub fn delete_relationship(
            &self,
            id: impl Into<String>,
            relationship_id: impl Into<String>,
        ) -> delete_relationship::RequestBuilder {
            delete_relationship::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                traceparent: None,
                tracestate: None,
                if_match: None,
            }
        }
        #[doc = "Retrieves the relationships from a digital twin.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn list_relationships(&self, id: impl Into<String>) -> list_relationships::RequestBuilder {
            list_relationships::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
                relationship_name: None,
            }
        }
        #[doc = "Retrieves all incoming relationship for a digital twin.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn list_incoming_relationships(&self, id: impl Into<String>) -> list_incoming_relationships::RequestBuilder {
            list_incoming_relationships::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Sends telemetry on behalf of a digital twin.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or message id is invalid.\n  * ValidationFailed - The telemetry content is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `telemetry`: The telemetry measurements to send from the digital twin."]
        #[doc = "* `message_id`: A unique message identifier (in the scope of the digital twin id) that is commonly used for de-duplicating messages."]
        pub fn send_telemetry(
            &self,
            id: impl Into<String>,
            telemetry: impl Into<serde_json::Value>,
            message_id: impl Into<String>,
        ) -> send_telemetry::RequestBuilder {
            send_telemetry::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                telemetry: telemetry.into(),
                message_id: message_id.into(),
                traceparent: None,
                tracestate: None,
                telemetry_source_time: None,
            }
        }
        #[doc = "Sends telemetry on behalf of a component in a digital twin.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id, message id, or component path is invalid.\n  * ValidationFailed - The telemetry content is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n  * ComponentNotFound - The component path was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `component_path`: The name of the DTDL component."]
        #[doc = "* `telemetry`: The telemetry measurements to send from the digital twin's component."]
        #[doc = "* `message_id`: A unique message identifier (in the scope of the digital twin id) that is commonly used for de-duplicating messages."]
        pub fn send_component_telemetry(
            &self,
            id: impl Into<String>,
            component_path: impl Into<String>,
            telemetry: impl Into<serde_json::Value>,
            message_id: impl Into<String>,
        ) -> send_component_telemetry::RequestBuilder {
            send_component_telemetry::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                component_path: component_path.into(),
                telemetry: telemetry.into(),
                message_id: message_id.into(),
                traceparent: None,
                tracestate: None,
                telemetry_source_time: None,
            }
        }
        #[doc = "Retrieves a component from a digital twin.\nStatus codes:\n* 200 OK\n* 400 Bad Request\n  * InvalidArgument - The digital twin id or component path is invalid.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n  * ComponentNotFound - The component path was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `component_path`: The name of the DTDL component."]
        pub fn get_component(&self, id: impl Into<String>, component_path: impl Into<String>) -> get_component::RequestBuilder {
            get_component::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                component_path: component_path.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Updates a component on a digital twin.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * InvalidArgument - The digital twin id, component path, or payload is invalid.\n  * JsonPatchInvalid - The JSON Patch provided is invalid.\n  * ValidationFailed - Applying the patch results in an invalid digital twin.\n* 404 Not Found\n  * DigitalTwinNotFound - The digital twin was not found.\n* 412 Precondition Failed\n  * PreconditionFailed - The precondition check (If-Match or If-None-Match) failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `component_path`: The name of the DTDL component."]
        #[doc = "* `patch_document`: An update specification described by JSON Patch. Updates to property values and $model elements may happen in the same request. Operations are limited to add, replace and remove."]
        pub fn update_component(
            &self,
            id: impl Into<String>,
            component_path: impl Into<String>,
            patch_document: Vec<serde_json::Value>,
        ) -> update_component::RequestBuilder {
            update_component::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                component_path: component_path.into(),
                patch_document,
                traceparent: None,
                tracestate: None,
                if_match: None,
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DigitalTwin> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DigitalTwin = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DigitalTwin>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DigitalTwin>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DigitalTwin> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DigitalTwin = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) twin: serde_json::Value,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_none_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity does not already exist."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.twin)?;
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DigitalTwin>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DigitalTwin>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) patch_document: Vec<serde_json::Value>,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch_document)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod get_relationship_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Relationship> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Relationship = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/relationships/{}",
                    self.client.endpoint(),
                    &self.id,
                    &self.relationship_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Relationship>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Relationship>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add_relationship {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Relationship> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Relationship = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) relationship: serde_json::Value,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_none_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity does not already exist."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.relationship)?;
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/relationships/{}",
                    self.client.endpoint(),
                    &self.id,
                    &self.relationship_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Relationship>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Relationship>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update_relationship {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) patch_document: Vec<serde_json::Value>,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch_document)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/relationships/{}",
                    self.client.endpoint(),
                    &self.id,
                    &self.relationship_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod delete_relationship {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/relationships/{}",
                    self.client.endpoint(),
                    &self.id,
                    &self.relationship_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod list_relationships {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RelationshipCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RelationshipCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) relationship_name: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "The name of the relationship."]
            pub fn relationship_name(mut self, relationship_name: impl Into<String>) -> Self {
                self.relationship_name = Some(relationship_name.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::RelationshipCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(traceparent) = &this.traceparent {
                                    req.insert_header("traceparent", traceparent);
                                }
                                if let Some(tracestate) = &this.tracestate {
                                    req.insert_header("tracestate", tracestate);
                                }
                                if let Some(relationship_name) = &this.relationship_name {
                                    req.url_mut().query_pairs_mut().append_pair("relationshipName", relationship_name);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/digitaltwins/{}/relationships", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod list_incoming_relationships {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IncomingRelationshipCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::IncomingRelationshipCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::IncomingRelationshipCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(traceparent) = &this.traceparent {
                                    req.insert_header("traceparent", traceparent);
                                }
                                if let Some(tracestate) = &this.tracestate {
                                    req.insert_header("tracestate", tracestate);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/incomingrelationships",
                    self.client.endpoint(),
                    &self.id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod send_telemetry {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) telemetry: serde_json::Value,
            pub(crate) message_id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) telemetry_source_time: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "An RFC 3339 timestamp that identifies the time the telemetry was measured."]
            pub fn telemetry_source_time(mut self, telemetry_source_time: impl Into<String>) -> Self {
                self.telemetry_source_time = Some(telemetry_source_time.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.telemetry)?;
                        req.insert_header("message-id", &this.message_id);
                        if let Some(telemetry_source_time) = &this.telemetry_source_time {
                            req.insert_header("telemetry-source-time", telemetry_source_time);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/digitaltwins/{}/telemetry", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod send_component_telemetry {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) component_path: String,
            pub(crate) telemetry: serde_json::Value,
            pub(crate) message_id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) telemetry_source_time: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "An RFC 3339 timestamp that identifies the time the telemetry was measured."]
            pub fn telemetry_source_time(mut self, telemetry_source_time: impl Into<String>) -> Self {
                self.telemetry_source_time = Some(telemetry_source_time.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.telemetry)?;
                        req.insert_header("message-id", &this.message_id);
                        if let Some(telemetry_source_time) = &this.telemetry_source_time {
                            req.insert_header("telemetry-source-time", telemetry_source_time);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/components/{}/telemetry",
                    self.client.endpoint(),
                    &self.id,
                    &self.component_path
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod get_component {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Component> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Component = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) component_path: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/components/{}",
                    self.client.endpoint(),
                    &self.id,
                    &self.component_path
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Component>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Component>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update_component {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Weak Etag."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) component_path: String,
            pub(crate) patch_document: Vec<serde_json::Value>,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) if_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch_document)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/digitaltwins/{}/components/{}",
                    self.client.endpoint(),
                    &self.id,
                    &self.component_path
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
}
pub mod event_routes {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all event routes.\nStatus codes:\n* 200 OK"]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                traceparent: None,
                tracestate: None,
                max_items_per_page: None,
            }
        }
        #[doc = "Retrieves an event route.\nStatus codes:\n* 200 OK\n* 404 Not Found\n  * EventRouteNotFound - The event route was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for an event route. The id is unique within event routes and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Adds or replaces an event route.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * EventRouteEndpointInvalid - The endpoint provided does not exist or is not active.\n  * EventRouteFilterInvalid - The event route filter is invalid.\n  * EventRouteIdInvalid - The event route id is invalid.\n  * LimitExceeded - The maximum number of event routes allowed has been reached."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for an event route. The id is unique within event routes and case sensitive."]
        #[doc = "* `event_route`: The event route data"]
        pub fn add(&self, id: impl Into<String>, event_route: impl Into<models::EventRoute>) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                event_route: event_route.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Deletes an event route.\nStatus codes:\n* 204 No Content\n* 404 Not Found\n  * EventRouteNotFound - The event route was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for an event route. The id is unique within event routes and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EventRouteCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::EventRouteCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) max_items_per_page: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested number."]
            pub fn max_items_per_page(mut self, max_items_per_page: i64) -> Self {
                self.max_items_per_page = Some(max_items_per_page);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::EventRouteCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(traceparent) = &this.traceparent {
                                    req.insert_header("traceparent", traceparent);
                                }
                                if let Some(tracestate) = &this.tracestate {
                                    req.insert_header("tracestate", tracestate);
                                }
                                if let Some(max_items_per_page) = &this.max_items_per_page {
                                    req.insert_header("max-items-per-page", &max_items_per_page.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/eventroutes", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EventRoute> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::EventRoute = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/eventroutes/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EventRoute>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::EventRoute>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) event_route: models::EventRoute,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.event_route)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/eventroutes/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/eventroutes/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
}
pub mod import_jobs {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all import jobs.\nStatus codes:\n* 200 OK"]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                traceparent: None,
                tracestate: None,
                max_items_per_page: None,
            }
        }
        #[doc = "Retrieves an import job.\nStatus codes:\n* 200 OK\n* 404 Not Found\n  * ImportJobNotFound - The import job was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the import job. The id is unique within the service and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Creates an import job.\nStatus codes:\n* 201 Created\n* 400 Bad Request\n  * JobLimitReached - The maximum number of import jobs allowed has been reached.\n  * ValidationFailed - The import job request is not valid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the import job. The id is unique within the service and case sensitive."]
        #[doc = "* `import_job`: The import job being added."]
        pub fn add(&self, id: impl Into<String>, import_job: impl Into<models::ImportJob>) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                import_job: import_job.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Deletes an import job. This is simply used to remove a job id, so it may be reused later. It can not be used to stop entities from being imported.\nStatus codes:\n* 204 No Content\n* 400 Bad Request\n  * ValidationFailed - The import job request is not valid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the import job. The id is unique within the service and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
        #[doc = "Cancels an import job that is currently running. Service will stop any import operations triggered by the current import job that are in progress, and go to a cancelled state. Please note that this will leave your instance in an unknown state as there won't be any rollback operation. \nStatus codes:\n* 200 Request Accepted\n* 400 Bad Request\n  * ValidationFailed - The import job request is not valid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the import job. The id is unique within the service and case sensitive."]
        pub fn cancel(&self, id: impl Into<String>) -> cancel::RequestBuilder {
            cancel::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ImportJobCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ImportJobCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) max_items_per_page: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested number."]
            pub fn max_items_per_page(mut self, max_items_per_page: i64) -> Self {
                self.max_items_per_page = Some(max_items_per_page);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ImportJobCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(traceparent) = &this.traceparent {
                                    req.insert_header("traceparent", traceparent);
                                }
                                if let Some(tracestate) = &this.tracestate {
                                    req.insert_header("tracestate", tracestate);
                                }
                                if let Some(max_items_per_page) = &this.max_items_per_page {
                                    req.insert_header("max-items-per-page", &max_items_per_page.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/imports", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ImportJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ImportJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/imports/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ImportJob>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ImportJob>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod add {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ImportJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ImportJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) import_job: models::ImportJob,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.import_job)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/imports/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ImportJob>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ImportJob>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/imports/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod cancel {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ImportJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ImportJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/imports/{}/cancel", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ImportJob>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ImportJob>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod delete_jobs {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all deletion jobs. This may be useful to find a delete job that was previously requested, or to view a history of delete jobs that have run or are currently running on the instance.\nStatus codes:\n* 200 OK"]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                traceparent: None,
                tracestate: None,
                max_items_per_page: None,
            }
        }
        #[doc = "Initiates a job which deletes all models, twins, and relationships on the instance. Does not delete any other types of entities.\nStatus codes:\n* 202 Created\n* 400 Bad Request\n  * JobLimitReached - The maximum number of delete jobs allowed has been reached. \n  * ValidationFailed - Operation-Id already exists."]
        pub fn add(&self) -> add::RequestBuilder {
            add::RequestBuilder {
                client: self.0.clone(),
                traceparent: None,
                tracestate: None,
                operation_id: None,
                timeout_in_minutes: None,
            }
        }
        #[doc = "Retrieves a delete job.\nStatus codes:\n* 200 OK\n* 404 Not Found\n  * DeleteJobNotFound - The delete job was not found."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the delete job. The id is unique within the service and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                traceparent: None,
                tracestate: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeleteJobCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DeleteJobCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) max_items_per_page: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested number."]
            pub fn max_items_per_page(mut self, max_items_per_page: i64) -> Self {
                self.max_items_per_page = Some(max_items_per_page);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::DeleteJobCollection, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(traceparent) = &this.traceparent {
                                    req.insert_header("traceparent", traceparent);
                                }
                                if let Some(tracestate) = &this.tracestate {
                                    req.insert_header("tracestate", tracestate);
                                }
                                if let Some(max_items_per_page) = &this.max_items_per_page {
                                    req.insert_header("max-items-per-page", &max_items_per_page.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/deletions", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
    }
    pub mod add {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeleteJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DeleteJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The URL to track the status of the long running operation."]
            pub fn operation_location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("operation-location"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
            pub(crate) operation_id: Option<String>,
            pub(crate) timeout_in_minutes: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "ID for the operation's status monitor. The ID is generated if header was not passed by the client."]
            pub fn operation_id(mut self, operation_id: impl Into<String>) -> Self {
                self.operation_id = Some(operation_id.into());
                self
            }
            #[doc = "Desired timeout for the delete job. Once the specified timeout is reached, service will stop any delete operations triggered by the current delete job that are in progress, and go to a failed state. Please note that this will leave your instance in an unknown state as there won't be any rollback operation."]
            pub fn timeout_in_minutes(mut self, timeout_in_minutes: i32) -> Self {
                self.timeout_in_minutes = Some(timeout_in_minutes);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        if let Some(operation_id) = &this.operation_id {
                            req.insert_header("operation-id", operation_id);
                        }
                        if let Some(timeout_in_minutes) = &this.timeout_in_minutes {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("timeoutInMinutes", &timeout_in_minutes.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/deletions", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeleteJob>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeleteJob>>;
            #[doc = "Returns a future that polls the long running operation and checks for the state via `properties.provisioningState` in the response body."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{body_content::get_provisioning_state, get_retry_after, LroStatus},
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    loop {
                        let this = self.clone();
                        let response = this.send().await?;
                        let retry_after = get_retry_after(response.as_raw_response().headers());
                        let status = response.as_raw_response().status();
                        let body = response.into_body().await?;
                        let provisioning_state = get_provisioning_state(status, &body)?;
                        log::trace!("current provisioning_state: {provisioning_state:?}");
                        match provisioning_state {
                            LroStatus::Succeeded => return Ok(body),
                            LroStatus::Failed => return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string())),
                            LroStatus::Canceled => {
                                return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                            }
                            _ => {
                                sleep(retry_after).await;
                            }
                        }
                    }
                })
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeleteJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DeleteJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) traceparent: Option<String>,
            pub(crate) tracestate: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Identifies the request in a distributed tracing system."]
            pub fn traceparent(mut self, traceparent: impl Into<String>) -> Self {
                self.traceparent = Some(traceparent.into());
                self
            }
            #[doc = "Provides vendor-specific trace identification information and is a companion to traceparent."]
            pub fn tracestate(mut self, tracestate: impl Into<String>) -> Self {
                self.tracestate = Some(tracestate.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(traceparent) = &this.traceparent {
                            req.insert_header("traceparent", traceparent);
                        }
                        if let Some(tracestate) = &this.tracestate {
                            req.insert_header("tracestate", tracestate);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/jobs/deletions/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2023-10-31");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeleteJob>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DeleteJob>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
