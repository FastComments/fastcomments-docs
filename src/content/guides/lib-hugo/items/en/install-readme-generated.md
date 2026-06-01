Pick one of the two standard ways to add a Hugo theme component.

### Option A: Hugo Module (recommended)

From your site root:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Then add the import to your `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Option B: Theme component (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Then reference it from your `hugo.toml`. List it alongside your own theme; later entries win, so keep your theme first:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```