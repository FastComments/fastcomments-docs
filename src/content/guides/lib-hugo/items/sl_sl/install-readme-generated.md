Izberite enega izmed dveh standardnih načinov za dodajanje komponente teme Hugo.

### Možnost A: Hugo modul (priporočeno)

Iz korenske mape vaše strani:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Nato dodajte uvoz v vašo datoteko `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Možnost B: Komponenta teme (Git podmodul)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Nato se nanj sklicujte iz vaše datoteke `hugo.toml`. Navedite ga skupaj z vašo lastno temo; kasnejši vnosi imajo prednost, zato naj bo vaša tema prva:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```