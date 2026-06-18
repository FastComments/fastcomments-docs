Wählen Sie eine der beiden Standardmethoden, um eine Hugo-Theme-Komponente hinzuzufügen.

### Option A: Hugo-Modul (empfohlen)

Aus dem Stammverzeichnis Ihrer Website:

```bash
hugo mod init github.com/you/your-site   # nur wenn Ihre Website noch kein Modul ist
hugo mod get github.com/FastComments/fastcomments-hugo
```

Dann fügen Sie den Import zu Ihrer `hugo.toml` hinzu:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Option B: Theme-Komponente (Git-Submodul)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Verweisen Sie dann darauf in Ihrer `hugo.toml`. Listen Sie es zusammen mit Ihrem eigenen Theme auf; spätere Einträge haben Vorrang, daher sollte Ihr Theme zuerst stehen:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```