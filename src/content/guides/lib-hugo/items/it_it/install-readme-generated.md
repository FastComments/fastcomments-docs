Scegli uno dei due metodi standard per aggiungere un componente del tema Hugo.

### Opzione A: Modulo Hugo (consigliato)

Dalla directory principale del tuo sito:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Quindi aggiungi l'import nel tuo `hugo.toml`:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Opzione B: Componente del tema (sottomodulo Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Poi fai riferimento a esso dal tuo `hugo.toml`. Elencalo insieme al tuo tema; le voci successive hanno la precedenza, quindi mantieni il tuo tema per primo:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```