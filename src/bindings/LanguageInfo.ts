// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CodeMirrorMode } from "./CodeMirrorMode";

/**
 * Programming language information.
 */
export type LanguageInfo = {
  /**
   * Programming language name.
   */
  name: string;
  /**
   * CodeMirror mode to use for the language.
   */
  codemirror_mode: CodeMirrorMode | null;
  /**
   * File extension for files in this language.
   */
  file_extension: string | null;
  /**
   * MIME type for files in this language.
   */
  mimetype: string | null;
  /**
   * Pygments lexer for syntax highlighting.
   */
  pygments_lexer: string | null;
};