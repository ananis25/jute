//! JSON schema for the Jupyter notebook `.ipynb` file format and Jute's
//! extensions.
//!
//! This file is based on the official [nbformat v4].
//!
//! [nbformat v4]: https://github.com/jupyter/nbformat/blob/v5.10.4/nbformat/v4/nbformat.v4.schema.json

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use ts_rs::TS;

/// Represents the root structure of a Jupyter Notebook file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct NotebookRoot {
    /// Root-level metadata of the notebook.
    pub metadata: NotebookMetadata,

    /// Notebook format (minor number). Incremented for backward-compatible
    /// changes.
    pub nbformat_minor: u8,

    /// Notebook format (major number). Incremented for incompatible changes.
    pub nbformat: u8,

    /// Array of cells in the notebook.
    pub cells: Vec<Cell>,
}

/// Root-level metadata for the notebook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct NotebookMetadata {
    /// Kernel information.
    #[ts(optional)]
    pub kernelspec: Option<KernelSpec>,

    /// Programming language information.
    #[ts(optional)]
    pub language_info: Option<LanguageInfo>,

    /// Original notebook format before conversion.
    #[ts(optional)]
    pub orig_nbformat: Option<u8>,

    /// Title of the notebook document.
    #[ts(optional)]
    pub title: Option<String>,

    /// Authors of the notebook document.
    #[ts(optional)]
    pub authors: Option<Vec<Author>>,

    /// Additional unrecognized attributes in metadata.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Kernel specification metadata.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct KernelSpec {
    /// Name of the kernel specification.
    pub name: String,

    /// Display name of the kernel.
    pub display_name: String,

    /// Additional unrecognized attributes in kernel specification.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Programming language information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct LanguageInfo {
    /// Programming language name.
    pub name: String,

    /// CodeMirror mode to use for the language.
    #[ts(optional)]
    pub codemirror_mode: Option<CodeMirrorMode>,

    /// File extension for files in this language.
    #[ts(optional)]
    pub file_extension: Option<String>,

    /// MIME type for files in this language.
    #[ts(optional)]
    pub mimetype: Option<String>,

    /// Pygments lexer for syntax highlighting.
    #[ts(optional)]
    pub pygments_lexer: Option<String>,

    /// Additional unrecognized attributes in language information.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Represents the CodeMirror mode, which could be a string or a nested object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
#[serde(untagged)]
pub enum CodeMirrorMode {
    /// String representation of the CodeMirror mode.
    String(String),
    /// Nested object representation of the CodeMirror mode.
    Object(BTreeMap<String, Value>),
}

/// Author information for the notebook document.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct Author {
    /// Name of the author.
    #[ts(optional)]
    pub name: Option<String>,

    /// Additional unrecognized attributes for authors.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Represents a notebook cell, which can be a raw, markdown, or code cell.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
#[serde(tag = "cell_type", rename_all = "snake_case")]
pub enum Cell {
    /// Raw cell type.
    Raw(RawCell),

    /// Markdown cell type.
    Markdown(MarkdownCell),

    /// Code cell type.
    Code(CodeCell),
}

/// Raw cell in the notebook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct RawCell {
    /// Identifier of the cell.
    #[ts(optional)]
    pub id: Option<String>,

    /// Metadata for the cell.
    pub metadata: CellMetadata,

    /// Content of the cell.
    pub source: MultilineString,

    /// Attachments (e.g., images) in the cell.
    #[ts(optional)]
    pub attachments: Option<CellAttachments>,
}

/// Markdown cell in the notebook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct MarkdownCell {
    /// Identifier of the cell.
    #[ts(optional)]
    pub id: Option<String>,

    /// Metadata for the cell.
    pub metadata: CellMetadata,

    /// Content of the cell.
    pub source: MultilineString,

    /// Attachments (e.g., images) in the cell.
    #[ts(optional)]
    pub attachments: Option<CellAttachments>,
}

/// Code cell in the notebook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct CodeCell {
    /// Identifier of the cell.
    #[ts(optional)]
    pub id: Option<String>,

    /// Metadata for the cell.
    pub metadata: CellMetadata,

    /// Content of the cell.
    pub source: MultilineString,

    /// Execution count of the cell (null if not executed).
    pub execution_count: Option<u32>,

    /// Outputs from executing the cell.
    pub outputs: Vec<Output>,
}

/// Metadata for a cell.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct CellMetadata {
    /// Additional unrecognized attributes in cell metadata.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Attachments for a cell, represented as MIME bundles keyed by filenames.
pub type CellAttachments = BTreeMap<String, MimeBundle>;

/// MIME bundle for representing various types of data.
pub type MimeBundle = BTreeMap<String, Value>;

/// Represents a string or array of strings (multiline).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
#[serde(untagged)]
pub enum MultilineString {
    /// Single-line string.
    Single(String),

    /// Multi-line array of strings.
    Multi(Vec<String>),
}

impl From<MultilineString> for String {
    fn from(m: MultilineString) -> Self {
        match m {
            MultilineString::Single(s) => s,
            MultilineString::Multi(v) if v.len() == 1 => v.into_iter().next().unwrap(),
            MultilineString::Multi(v) => v.join(""),
        }
    }
}

impl MultilineString {
    /// Convert a string to a multiline string, mimicking Jupyter.
    ///
    /// Usually, we could just use `MultilineString::Single`, but Jupyter's
    /// behavior is to always return an array, so we respect that. It also
    /// breaks strings after newline characters.
    pub fn normalize(&self) -> Self {
        let value = match self {
            MultilineString::Single(s) => s,
            MultilineString::Multi(v) => &v.join(""),
        };

        let mut lines = Vec::new();
        let mut remaining = &value[..];
        while !remaining.is_empty() {
            let next_break = remaining.find('\n').map_or(remaining.len(), |i| i + 1);
            lines.push(remaining[..next_break].to_string());
            remaining = &remaining[next_break..];
        }
        MultilineString::Multi(lines)
    }
}

/// Output from executing a code cell.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
#[serde(tag = "output_type", rename_all = "snake_case")]
pub enum Output {
    /// Execution result output.
    ExecuteResult(OutputExecuteResult),

    /// Display data output.
    DisplayData(OutputDisplayData),

    /// Stream output.
    Stream(OutputStream),

    /// Error output.
    Error(OutputError),
}

/// Result of executing a code cell.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct OutputExecuteResult {
    /// Execution count of the result.
    pub execution_count: Option<u32>,

    /// Data returned by the execution.
    pub data: MimeBundle,

    /// Metadata associated with the result.
    pub metadata: OutputMetadata,

    /// Additional unrecognized attributes in execution results.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Display data output.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct OutputDisplayData {
    /// Data to display.
    pub data: MimeBundle,

    /// Metadata associated with the display data.
    pub metadata: OutputMetadata,

    /// Additional unrecognized attributes in display data.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Stream output.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct OutputStream {
    /// Name of the stream (e.g., stdout or stderr).
    pub name: String,

    /// Text content of the stream.
    pub text: MultilineString,

    /// Additional unrecognized attributes in stream output.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Error output.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
pub struct OutputError {
    /// Name of the error.
    pub ename: String,

    /// Value or message of the error.
    pub evalue: String,

    /// Traceback of the error.
    pub traceback: Vec<String>,

    /// Additional unrecognized attributes in error output.
    #[serde(flatten)]
    #[ts(skip)]
    pub other: Map<String, Value>,
}

/// Metadata associated with outputs.
pub type OutputMetadata = BTreeMap<String, Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_notebook() {
        let json = r#"
            {
                "metadata": {
                    "kernelspec": {
                        "name": "python3",
                        "display_name": "Python 3"
                    },
                    "language_info": {
                        "name": "python",
                        "codemirror_mode": {
                            "name": "ipython",
                            "version": 3
                        },
                        "file_extension": ".py",
                        "mimetype": "text/x-python",
                        "pygments_lexer": "ipython3",
                        "version": "3.8.5",
                        "nbconvert_exporter": "python"
                    },
                    "orig_nbformat": 4,
                    "title": "Example Notebook",
                    "authors": [
                        {
                            "name": "Alice"
                        },
                        {
                            "name": "Bob"
                        }
                    ],
                    "custom": "metadata"
                },
                "nbformat_minor": 4,
                "nbformat": 4,
                "cells": [
                    {
                        "cell_type": "code",
                        "id": "cell-1",
                        "metadata": {
                            "custom": "metadata"
                        },
                        "source": "print('Hello, world!')",
                        "execution_count": 1,
                        "outputs": [
                            {
                                "output_type": "execute_result",
                                "execution_count": 1,
                                "data": {
                                    "text/plain": "Hello, world!"
                                },
                                "metadata": {
                                    "custom": "metadata"
                                }
                            }
                        ]
                    }
                ]
            }
        "#;

        let notebook: NotebookRoot = serde_json::from_str(json).unwrap();
        assert_eq!(
            notebook.metadata.kernelspec.as_ref().unwrap().name,
            "python3"
        );
        assert_eq!(
            notebook.metadata.language_info.as_ref().unwrap().name,
            "python"
        );
        assert_eq!(notebook.metadata.orig_nbformat, Some(4));
        assert_eq!(
            notebook.metadata.title.as_ref().unwrap(),
            "Example Notebook"
        );
        assert_eq!(
            notebook.metadata.authors.as_ref().unwrap()[0]
                .name
                .as_ref()
                .unwrap(),
            "Alice"
        );
        assert_eq!(
            notebook.metadata.authors.as_ref().unwrap()[1]
                .name
                .as_ref()
                .unwrap(),
            "Bob"
        );
        assert_eq!(notebook.metadata.other.get("custom").unwrap(), "metadata");
        assert_eq!(notebook.nbformat_minor, 4);
        assert_eq!(notebook.nbformat, 4);
        assert_eq!(notebook.cells.len(), 1);
    }

    #[test]
    fn string_to_multiline() {
        let empty = MultilineString::Single("".into()).normalize();
        assert_eq!(empty, MultilineString::Multi(vec![]));

        let single = MultilineString::Single("Hello, world!".into()).normalize();
        assert_eq!(
            single,
            MultilineString::Multi(vec!["Hello, world!".to_string()])
        );

        let multi = MultilineString::Single("Hello,\nworld!".into()).normalize();
        assert_eq!(
            multi,
            MultilineString::Multi(vec!["Hello,\n".to_string(), "world!".to_string()])
        );

        let multi = MultilineString::Single("Hello,\n\nworld!\n".into()).normalize();
        assert_eq!(
            multi,
            MultilineString::Multi(vec![
                "Hello,\n".to_string(),
                "\n".to_string(),
                "world!\n".to_string()
            ])
        );
    }
}
