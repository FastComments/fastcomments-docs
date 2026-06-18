---
Définissez votre ID de tenant une fois dans `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # remplacez "demo" par l'ID de votre tenant FastComments
```

Ensuite, soit intégrez le widget de commentaires dans votre thème (voir [Intégration du thème](#theme-integration-readme-generated)), soit insérez un shortcode dans le Markdown de n'importe quelle page :

```text
\{{< fastcomments >}}
```
---