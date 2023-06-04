use crate::ConfigBuilder;

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
;; Note that `php-mode' assumes php code is separated from HTML code,
;; following modern Laravel best practices. If you'd like to combine
;; PHP and HTML in a single file, you may prefer `web-mode'.
(use-package php-mode
  :ensure t)
"#;

const TSX: &str = r#"
;; Typescript, JS, & React support is provided by web-mode. This differs
;; from other programming languages because TSX is really multi-language
;; by design, including both TypeScript and HTML in a single file. I've
;; found web-mode provides the best TSX experience and works equally well
;; for plain ol' JS or TS.
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

pub struct Languages {}

impl ConfigBuilder for Languages {
    fn build_string(options: Option<Vec<String>>) -> String {
        let languages = options.unwrap_or_default();
        let mut result = String::new();

        for lang in languages {
            let config = match lang.as_str() {
                "go" => Some(GO),
                "lua" => Some(LUA),
                "markdown" => Some(MARKDOWN),
                "php" => Some(PHP),
                "tsx" => Some(TSX),
                "rust" => Some(RUST),
                "yaml" => Some(YAML),
                _ => None,
            };

            if let Some(config) = config {
                result = result + config;
            }
        }

        result
    }
}
