/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::path::PathBuf;

use hurl_core::ast::SourceInfo;

use crate::http::{HttpError, RequestedHttpVersion};

/// Represents a single instance of a runtime error, usually triggered by running a
/// [`hurl_core::ast::Entry`]. Running a Hurl content (see [`crate::runner::run`]) returns a list of
/// result for each entry. Each entry result can contain a list of [`Error`]. The runtime error variant
/// is defined in [`RunnerError`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: RunnerError,
    pub assert: bool,
}

impl Error {
    pub fn new(source_info: SourceInfo, inner: RunnerError, assert: bool) -> Error {
        Error {
            source_info,
            inner,
            assert,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerError {
    AssertBodyValueError {
        actual: String,
        expected: String,
    },
    AssertFailure {
        actual: String,
        expected: String,
        type_mismatch: bool,
    },
    AssertHeaderValueError {
        actual: String,
    },
    AssertStatus {
        actual: String,
    },
    AssertVersion {
        actual: String,
    },
    CouldNotUncompressResponse(String),
    /// I/O read error on `path`.
    FileReadAccess {
        path: PathBuf,
    },
    /// I/O write error on `path`.
    FileWriteAccess {
        path: PathBuf,
        error: String,
    },
    FilterDecode(String),
    FilterInvalidEncoding(String),
    FilterInvalidInput(String),
    FilterMissingInput,
    HttpConnection(String),
    InvalidJson {
        value: String,
    },
    InvalidCharset {
        charset: String,
    },
    InvalidDecoding {
        charset: String,
    },
    InvalidRegex,
    InvalidUrl(String, String),
    NoQueryResult,
    QueryHeaderNotFound,
    QueryInvalidJsonpathExpression {
        value: String,
    },
    QueryInvalidXpathEval,
    QueryInvalidXml,
    QueryInvalidJson,
    TemplateVariableNotDefined {
        name: String,
    },
    TemplateVariableInvalidType {
        name: String,
        value: String,
        expecting: String,
    },
    TooManyRedirect,
    UnsupportedContentEncoding(String),
    UnsupportedHttpVersion(RequestedHttpVersion),
    UnrenderableVariable {
        name: String,
        value: String,
    },
    /// Unauthorized file access, check `--file-root` option.
    UnauthorizedFileAccess {
        path: PathBuf,
    },
}

/// Textual Output for runner errors
impl hurl_core::error::Error for Error {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match &self.inner {
            RunnerError::AssertBodyValueError { .. } => "Assert body value".to_string(),
            RunnerError::AssertFailure { .. } => "Assert failure".to_string(),
            RunnerError::AssertHeaderValueError { .. } => "Assert header value".to_string(),
            RunnerError::AssertStatus { .. } => "Assert status code".to_string(),
            RunnerError::AssertVersion { .. } => "Assert HTTP version".to_string(),
            RunnerError::CouldNotUncompressResponse(..) => "Decompression error".to_string(),
            RunnerError::FileReadAccess { .. } => "File read access".to_string(),
            RunnerError::FileWriteAccess { .. } => "File write access".to_string(),
            RunnerError::FilterDecode { .. } => "Filter error".to_string(),
            RunnerError::FilterInvalidEncoding { .. } => "Filter error".to_string(),
            RunnerError::FilterInvalidInput { .. } => "Filter error".to_string(),
            RunnerError::FilterMissingInput => "Filter error".to_string(),
            RunnerError::HttpConnection { .. } => "HTTP connection".to_string(),
            RunnerError::InvalidCharset { .. } => "Invalid charset".to_string(),
            RunnerError::InvalidDecoding { .. } => "Invalid decoding".to_string(),
            RunnerError::InvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerError::InvalidRegex => "Invalid regex".to_string(),
            RunnerError::InvalidUrl(..) => "Invalid URL".to_string(),
            RunnerError::NoQueryResult => "No query result".to_string(),
            RunnerError::QueryHeaderNotFound => "Header not found".to_string(),
            RunnerError::QueryInvalidJson => "Invalid JSON".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { .. } => "Invalid JSONPath".to_string(),
            RunnerError::QueryInvalidXml => "Invalid XML".to_string(),
            RunnerError::QueryInvalidXpathEval => "Invalid XPath expression".to_string(),
            RunnerError::TemplateVariableInvalidType { .. } => "Invalid variable type".to_string(),
            RunnerError::TemplateVariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerError::TooManyRedirect => "HTTP connection".to_string(),
            RunnerError::UnauthorizedFileAccess { .. } => "Unauthorized file access".to_string(),
            RunnerError::UnrenderableVariable { .. } => "Unrenderable variable".to_string(),
            RunnerError::UnsupportedContentEncoding(..) => "Decompression error".to_string(),
            RunnerError::UnsupportedHttpVersion(..) => "Unsupported HTTP version".to_string(),
        }
    }

    fn fixme(&self, content: &[&str]) -> String {
        match &self.inner {
            RunnerError::AssertBodyValueError { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::AssertFailure {
                actual,
                expected,
                type_mismatch,
                ..
            } => {
                let additional = if *type_mismatch {
                    "\n   >>> types between actual and expected are not consistent"
                } else {
                    ""
                };
                format!("   actual:   {actual}\n   expected: {expected}{additional}")
            }
            RunnerError::AssertHeaderValueError { actual } => {
                let message = &format!("actual value is <{actual}>");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::AssertStatus { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::AssertVersion { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::CouldNotUncompressResponse(algorithm) => {
                let message = &format!("could not uncompress response with {algorithm}");

                // only add carets if source_info is set
                // TODO: add additional attribute in the error to be more explicit?
                if self.source_info.start.line == 0 {
                    message.to_string()
                } else {
                    hurl_core::error::add_carets(message, self.source_info, content)
                }
            }
            RunnerError::FileReadAccess { path } => {
                let message = &format!("file {} can not be read", path.to_string_lossy());
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::FileWriteAccess { path, error } => {
                format!("{} can not be written ({error})", path.to_string_lossy())
            }
            RunnerError::FilterDecode(encoding) => {
                let message = &format!("value can not be decoded with <{encoding}> encoding");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::FilterInvalidEncoding(encoding) => {
                let message = &format!("<{encoding}> encoding is not supported");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::FilterInvalidInput(message) => {
                let message = &format!("invalid filter input: {message}");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::FilterMissingInput => {
                let message = "missing value to apply filter";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::HttpConnection(message) => {
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::InvalidCharset { charset } => {
                let message = &format!("the charset '{charset}' is not valid");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::InvalidDecoding { charset } => {
                let message = &format!("the body can not be decoded with charset '{charset}'");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::InvalidJson { value } => {
                let message = &format!("actual value is <{value}>");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::InvalidRegex => {
                let message = "regex expression is not valid";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::InvalidUrl(url, reason) => {
                let message = &format!("invalid URL <{url}> ({reason})");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::NoQueryResult => {
                let message = "The query didn't return any result";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::QueryHeaderNotFound => {
                let message = "this header has not been found in the response";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::QueryInvalidJson => {
                let message = "the HTTP response is not a valid JSON";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::QueryInvalidJsonpathExpression { value } => {
                let message = &format!("the JSONPath expression '{value}' is not valid");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::QueryInvalidXml => {
                let message = "the HTTP response is not a valid XML";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::QueryInvalidXpathEval => {
                let message = "the XPath expression is not valid";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::TemplateVariableInvalidType {
                value, expecting, ..
            } => {
                let message = &format!("expecting {expecting}, actual value is <{value}>");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::TemplateVariableNotDefined { name } => {
                let message = &format!("you must set the variable {name}");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::TooManyRedirect => {
                let message = "too many redirect";
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::UnauthorizedFileAccess { path } => {
                let message = &format!(
                    "unauthorized access to file {}, check --file-root option",
                    path.to_string_lossy()
                );
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::UnrenderableVariable { name, value } => {
                let message = &format!("variable <{name}> with value {value} can not be rendered");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::UnsupportedContentEncoding(algorithm) => {
                let message = &format!("compression {algorithm} is not supported");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
            RunnerError::UnsupportedHttpVersion(version) => {
                let message = &format!("{version} is not supported, check --version");
                hurl_core::error::add_carets(message, self.source_info, content)
            }
        }
    }

    fn show_source_line(&self) -> bool {
        true
    }
}

impl From<HttpError> for RunnerError {
    /// Converts a HttpError to a RunnerError.
    fn from(item: HttpError) -> Self {
        match item {
            HttpError::CouldNotParseResponse => {
                RunnerError::HttpConnection("could not parse Response".to_string())
            }
            HttpError::CouldNotUncompressResponse { description } => {
                RunnerError::CouldNotUncompressResponse(description)
            }
            HttpError::InvalidCharset { charset } => RunnerError::InvalidCharset { charset },
            HttpError::InvalidDecoding { charset } => RunnerError::InvalidDecoding { charset },
            HttpError::InvalidUrl(url, reason) => RunnerError::InvalidUrl(url, reason),
            HttpError::Libcurl { code, description } => {
                RunnerError::HttpConnection(format!("({code}) {description}"))
            }
            HttpError::LibcurlUnknownOption {
                option,
                minimum_version,
            } => RunnerError::HttpConnection(format!(
                "Option {option} requires libcurl version {minimum_version} or higher"
            )),
            HttpError::StatuslineIsMissing => {
                RunnerError::HttpConnection("status line is missing".to_string())
            }
            HttpError::TooManyRedirect => RunnerError::TooManyRedirect,
            HttpError::UnsupportedContentEncoding { description } => {
                RunnerError::UnsupportedContentEncoding(description)
            }
            HttpError::UnsupportedHttpVersion(version) => {
                RunnerError::UnsupportedHttpVersion(version)
            }
        }
    }
}
