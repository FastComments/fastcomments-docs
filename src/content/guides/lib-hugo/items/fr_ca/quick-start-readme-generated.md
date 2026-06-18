---
Définissez votre ID de locataire une fois dans `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # remplacez "demo" par votre ID de locataire FastComments
```

Ensuite, branchez soit le widget de commentaires à votre thème (voir [Intégration du thème](#theme-integration-readme-generated)), soit insérez un shortcode dans le Markdown de n'importe quelle page :

```text
\{{< fastcomments >}}
```
---