use crate::ConfigBuilder;

const HELPFUL: &str = r#"
;; Adds contextual information to Emacs help documentation, making it
;; easier to locate code examples and see sample usage when looking
;; up functions or variables.
;; https://github.com/Wilfred/helpful
(use-package helpful
  :ensure t
  :bind (("C-h f" . #'helpful-callable)
         ("C-h v" . #'helpful-variable)
         ("C-h k" . #'helpful-key)
         ("C-c C-d" . #'helpful-at-point)
         ("C-h F" . #'helpful-function)
         ("C-h C" . #'helpful-command)))
"#;

const VIM: &str = r#"
;; Adds vim emulation to Emacs. Activating `evil-mode' (which the below does
;; for all buffers) will swap your keybindings to modal-based vim. That said,
;; Emacs is a lot more than just editing text. Evil Collection provides a
;; compatibility layer between Emacs packages and vim keybindings that may not
;; exist in `evil-mode'. A good example is `dired': with Evil Collection
;; you can navigate files and directories with vim keybindings instead of the
;; usual C-p & C-n.
;; https://github.com/emacs-evil/evil
(use-package evil
  :ensure t
  :init
  (setq evil-want-integration t)
  (setq evil-want-keybinding nil)
  :config
  (evil-mode 1))

;; Adds better Evil defaults for most built-in packages
(use-package evil-collection
  :after evil
  :ensure t
  :config
  (evil-collection-init))
"#;

const DENOTE: &str = r#"
;; `org-mode' is great but Denote makes it even better. Denote helps organize
;; your org (or plain text/markdown) notes with a intelligent naming scheme
;; and some helpful utilities. I highly recommend reading the manual:
;; https://protesilaos.com/emacs/denote
(use-package denote
  :ensure t
  :custom
  (denote-known-keywords '("emacs" "journal"))
  ;; This directory is where your notes live.
  (denote-directory (expand-file-name "~/denote/"))
  :bind
  (("C-c n n" . denote)
   ("C-c n f" . denote-open-or-create)
   ("C-c n i" . denote-link)))
"#;

const MAGIT: &str = r#"
;; Adds a fully-featured git client to Emacs. After working with Magit you'll
;; never want to go back to the git CLI. Activate it with `C-c g`.
;; https://magit.vc/
(use-package magit
  :ensure t
  :bind (("C-c g" . magit-status)))
"#;

pub struct Features {}

impl ConfigBuilder for Features {
    fn build_string(options: Option<Vec<String>>) -> String {
        let features = options.unwrap_or_default();
        let mut result = String::new();

        for feat in features {
            let config = match feat.as_str() {
                "helpful" => Some(HELPFUL),
                "vim" => Some(VIM),
                "denote" => Some(DENOTE),
                "magit" => Some(MAGIT),
                _ => None,
            };

            if let Some(config) = config {
                result = result + config;
            }
        }

        result
    }
}
