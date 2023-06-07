use crate::ConfigBuilder;

const HELPFUL: &str = r#"
;; Helpful adds contextual information to Emacs help documentation, making
;; it easier to locate code examples and see sample usage when looking
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
;; Evil adds vim emulation to Emacs. Activating `evil-mode' will swap your
;; regular keybindings to modal editing with vim, which this use-package
;; call configures globally. Since there's a ton of keybindings that Evil
;; needs to modify it'll occasionally lack support for a particular 
;; package. Evil Collection fills in those missing gaps.
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
  ;; This is the directory where your notes live.
  (denote-directory (expand-file-name "~/denote/"))
  :bind
  (("C-c n n" . denote)
   ("C-c n f" . denote-open-or-create)
   ("C-c n i" . denote-link)))
"#;

const MAGIT: &str = r#"
;; Magit is a fully-featured git frontend. After working with Magit you may
;; never want to go back to the git CLI. Activate it with `C-c g`.
;; https://magit.vc/
(use-package magit
  :ensure t
  :bind (("C-c g" . magit-status)))
"#;

const BREADCRUMBS: &str = r#"
;; You can install packages from version control with use-package and
;; the :vc keyword argument. For the list of supported fetchers, view
;; the documentation for the variable `vc-use-package-fetchers'.
;;
;; Breadcrumb adds breadcrumbs to the top of your open buffers and works
;; great with project.el, Emacs's built-in project management package.
;; https://www.gnu.org/software/emacs/manual/html_node/emacs/Projects.html
;; https://github.com/joaotavora/breadcrumb
(use-package breadcrumb
  :vc (:fetcher github :repo joaotavora/breadcrumb)
  :init (breadcrumb-mode))
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
                "breadcrumbs" => Some(BREADCRUMBS),
                _ => None,
            };

            if let Some(config) = config {
                result = result + config;
            }
        }

        result
    }
}
