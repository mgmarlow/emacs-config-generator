use crate::ConfigBuilder;

const HELPFUL: &str = r#"
;; Add extra context to Emacs documentation to help make it easier to
;; search and understand. This configuration uses the keybindings 
;; recommended by the package author.
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
;; Adds vim emulation. Activate `evil-mode' to swap your default Emacs
;; keybindings with the modal editor of great infamy. There's a ton of
;; keybindings that Evil needs to modify, so this configuration also
;; includes `evil-collection' to fill in the gaps.
(use-package evil
  :ensure t
  :init
  (setq evil-want-integration t)
  (setq evil-want-keybinding nil)
  :config
  (evil-mode 1))

(use-package evil-collection
  :after evil
  :ensure t
  :config
  (evil-collection-init))
"#;

const DENOTE: &str = r#"
;; `org-mode' is great but Denote makes it even better by adding
;; features that you'd find in something like Obsidian (like
;; backlinks!). You can write your notes in org, markdown, or plain
;; text, though I recommend giving `org-mode' a try if you've never
;; used it before. The Denote manual is also excellent:
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
;; An extremely feature-rich git client. Activate it with "C-c g".
(use-package magit
  :ensure t
  :bind (("C-c g" . magit-status)))
"#;

const BREADCRUMBS: &str = r#"
;; In addition to installing packages from the configured package
;; registries, you can also install straight from version control
;; with the :vc keyword argument. For the full list of supported
;; fetchers, view the documentation for the variable
;; `vc-use-package-fetchers'.
;;
;; Breadcrumb adds, well, breadcrumbs to the top of your open buffers
;; and works great with project.el, the Emacs project manager.
;;
;; Read more about projects here:
;; https://www.gnu.org/software/emacs/manual/html_node/emacs/Projects.html
(use-package breadcrumb
  :vc (:fetcher github :repo joaotavora/breadcrumb)
  :init (breadcrumb-mode))
"#;

const LISP: &str = r#"
;; As you've probably noticed, Lisp has a lot of parentheses.
;; Maintaining the syntactical correctness of these parentheses
;; can be a pain when you're first getting started with Lisp,
;; especially when you're fighting the urge to break up groups
;; of closing parens into separate lines. Luckily we have
;; Paredit, a package that maintains the structure of your
;; parentheses for you. At first, Paredit might feel a little
;; odd; you'll probably need to look at a tutorial (linked
;; below) or read the docs before you can use it effectively.
;; But once you pass that initial barrier you'll write Lisp
;; code like it's second nature.
;; http://danmidwood.com/content/2014/11/21/animated-paredit.html
;; https://stackoverflow.com/a/5243421/3606440
(use-package paredit
  :ensure t
  :hook ((emacs-lisp-mode . enable-paredit-mode)
         (lisp-mode . enable-paredit-mode)
         (ielm-mode . enable-paredit-mode)
         (lisp-interaction-mode . enable-paredit-mode)
         (scheme-mode . enable-paredit-mode)))
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
                "lisp" => Some(LISP),
                _ => None,
            };

            if let Some(config) = config {
                result += config;
            }
        }

        result
    }
}
