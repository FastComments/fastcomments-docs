Choisissez l'une des deux méthodes standard pour ajouter un composant de thème Hugo.

### Option A : Module Hugo (recommandé)

Depuis la racine de votre site :

```bash
hugo mod init github.com/you/your-site   # uniquement si votre site n'est pas déjà un module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Ajoutez ensuite l'import dans votre `hugo.toml` :

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Option B : Composant de thème (sous-module Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Référencez-le ensuite depuis votre `hugo.toml`. Listez-le aux côtés de votre propre thème ; les entrées suivantes ont priorité, donc placez votre thème en premier :

```toml
theme = ["your-theme", "fastcomments-hugo"]
```