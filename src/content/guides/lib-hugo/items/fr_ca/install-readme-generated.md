Choisissez l'une des deux façons standard d'ajouter un composant de thème Hugo.

### Option A : Module Hugo (recommandé)

Depuis la racine de votre site :

```bash
hugo mod init github.com/you/your-site   # seulement si votre site n'est pas déjà un module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Ensuite, ajoutez l'importation à votre `hugo.toml` :

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Option B : Composant de thème (sous-module Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Ensuite, référencez-le depuis votre `hugo.toml`. Énumérez-le aux côtés de votre propre thème ; les entrées plus tardives l'emportent, donc gardez votre thème en premier :

```toml
theme = ["your-theme", "fastcomments-hugo"]
```