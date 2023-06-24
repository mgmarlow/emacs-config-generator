use crate::ConfigBuilder;

const ELIXIR: &str = r#"
(use-package elixir-mode
  :ensure t)
"#;

const GO: &str = r#"
(use-package go-mode
  :ensure t
  :bind (:map go-mode-map
	      ("C-c C-f" . 'gofmt))
  :hook (before-save . gofmt-before-save))
"#;

const LUA: &str = r#"
(use-package lua-mode
  :ensure t)
"#;

const JULIA: &str = r#"
(use-package julia-mode
  :ensure t)
"#;

const MARKDOWN: &str = r#"
(use-package markdown-mode
  :ensure t
  ;; These extra modes help clean up the Markdown editing experience.
  ;; `visual-line-mode' turns on word wrap and helps editing commands
  ;; work with paragraphs of text. `flyspell-mode' turns on an
  ;; automatic spell checker.
  :hook ((markdown-mode . visual-line-mode)
         (markdown-mode . flyspell-mode))
  :init
  (setq markdown-command "multimarkdown"))
"#;

const PHP: &str = r#"
;; Note that `php-mode' assumes php code is separate from HTML.
;; If you prefer working with PHP and HTML in a single file you
;; may prefer `web-mode'.
(use-package php-mode
  :ensure t)
"#;

const TSX: &str = r#"
;; TypeScript, JS, and JSX/TSX support.
(use-package web-mode
  :ensure t
  :mode (("\\.ts\\'" . web-mode)
         ("\\.js\\'" . web-mode)
         ("\\.mjs\\'" . web-mode)
         ("\\.tsx\\'" . web-mode)
         ("\\.jsx\\'" . web-mode))
  :custom
  (web-mode-content-types-alist '(("jsx" . "\\.js[x]?\\'")))
  (web-mode-code-indent-offset 2)
  (web-mode-css-indent-offset 2)
  (web-mode-markup-indent-offset 2)
  (web-mode-enable-auto-quoting nil))
"#;

const RUST: &str = r#"
(use-package rust-mode
  :ensure t
  :bind (:map rust-mode-map
	      ("C-c C-r" . 'rust-run)
	      ("C-c C-c" . 'rust-compile)
	      ("C-c C-f" . 'rust-format-buffer)
	      ("C-c C-t" . 'rust-test))
  :hook (rust-mode . prettify-symbols-mode))
"#;

const YAML: &str = r#"
(use-package yaml-mode
  :ensure t)
"#;

pub fn eglot(languages: Vec<String>) -> String {
    let hooks: Vec<String> = languages
        .iter()
        .filter_map(|l| match l.as_str() {
            "go" => Some(String::from("(go-mode . eglot-ensure)")),
            "tsx" => Some(String::from("(web-mode . eglot-ensure)")),
            "rust" => Some(String::from("(rust-mode . eglot-ensure)")),
            _ => None,
        })
        .collect();

    let hooks_extra: String = if hooks.len() > 0 {
        format!(
            r#"
  ;; Add your programming modes here to automatically start Eglot,
  ;; assuming you have the respective LSP server installed.
  :hook ({:})"#,
            hooks.join("\n         "),
        )
    } else {
        String::from("")
    };

    let web_mode_extra: &str = if languages.contains(&String::from("tsx")) {
        r#"
  :config
  ;; You can configure additional LSP servers by modifying
  ;; `eglot-server-programs'. The following tells eglot to use TypeScript
  ;; language server when working in `web-mode'.
  (add-to-list 'eglot-server-programs
               '(web-mode . ("typescript-language-server" "--stdio")))"#
    } else {
        ""
    };

    format!(
        r#"
;; Adds LSP support. Note that you must have the respective LSP
;; server installed on your machine to use it with Eglot. e.g.
;; rust-analyzer to use Eglot with `rust-mode'.
(use-package eglot
  :ensure t
  :bind (("s-<mouse-1>" . eglot-find-implementation)
         ("C-c ." . eglot-code-action-quickfix)){:}{:})
"#,
        hooks_extra, web_mode_extra
    )
}

pub struct Languages {}

impl ConfigBuilder for Languages {
    fn build_string(options: Option<Vec<String>>) -> String {
        let languages = options.unwrap_or_default();
        let mut result = String::new();

        for lang in languages {
            let config = match lang.as_str() {
                "elixir" => Some(ELIXIR),
                "go" => Some(GO),
                "julia" => Some(JULIA),
                "lua" => Some(LUA),
                "markdown" => Some(MARKDOWN),
                "php" => Some(PHP),
                "tsx" => Some(TSX),
                "rust" => Some(RUST),
                "yaml" => Some(YAML),
                _ => None,
            };

            if let Some(config) = config {
                result += config;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eglot() {
        assert_eq!(
            eglot(vec![]),
            r#"
;; Adds LSP support. Note that you must have the respective LSP
;; server installed on your machine to use it with Eglot. e.g.
;; rust-analyzer to use Eglot with `rust-mode'.
(use-package eglot
  :ensure t
  :bind (("s-<mouse-1>" . eglot-find-implementation)
         ("C-c ." . eglot-code-action-quickfix)))
"#,
        );

        assert_eq!(
            eglot(vec![String::from("go"), String::from("rust")]),
            r#"
;; Adds LSP support. Note that you must have the respective LSP
;; server installed on your machine to use it with Eglot. e.g.
;; rust-analyzer to use Eglot with `rust-mode'.
(use-package eglot
  :ensure t
  :bind (("s-<mouse-1>" . eglot-find-implementation)
         ("C-c ." . eglot-code-action-quickfix))
  ;; Add your programming modes here to automatically start Eglot,
  ;; assuming you have the respective LSP server installed.
  :hook ((go-mode . eglot-ensure)
         (rust-mode . eglot-ensure)))
"#,
        );

        assert_eq!(
            eglot(vec![String::from("tsx")]),
            r#"
;; Adds LSP support. Note that you must have the respective LSP
;; server installed on your machine to use it with Eglot. e.g.
;; rust-analyzer to use Eglot with `rust-mode'.
(use-package eglot
  :ensure t
  :bind (("s-<mouse-1>" . eglot-find-implementation)
         ("C-c ." . eglot-code-action-quickfix))
  ;; Add your programming modes here to automatically start Eglot,
  ;; assuming you have the respective LSP server installed.
  :hook ((web-mode . eglot-ensure))
  :config
  ;; You can configure additional LSP servers by modifying
  ;; `eglot-server-programs'. The following tells eglot to use TypeScript
  ;; language server when working in `web-mode'.
  (add-to-list 'eglot-server-programs
               '(web-mode . ("typescript-language-server" "--stdio"))))
"#,
        );
    }
}
