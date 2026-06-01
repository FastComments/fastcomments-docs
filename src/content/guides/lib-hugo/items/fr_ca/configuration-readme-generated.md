Toutes les options du widget FastComments sont définies sous `[params.fastcomments]` dans `hugo.toml`, et peuvent être remplacées par page dans le front matter sous `[fastcomments]`. Priorité, de la plus basse à la plus élevée : site params, front matter de la page, paramètres du shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter d'une page
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Lorsque ni `url` ni `urlId` n'est fourni, `url` prend par défaut le permalien de la page afin que les fils de commentaires restent liés à une URL stable.

### Résidence des données dans l'UE

Les clients de l'UE définissent `region = "eu"` pour acheminer le widget vers `cdn-eu.fastcomments.com` :

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Remarque sur la casse des clés

Hugo met en minuscules chaque clé dans `hugo.toml` et le front matter, mais les widgets FastComments exigent des clés en camelCase (`tenantId`, `hasDarkBackground`). Ce composant restaure automatiquement la casse correcte pour chaque option connue de premier niveau, donc écrivez les options dans leur forme camelCase normale. Les clés imbriquées à l'intérieur d'une valeur d'objet (par exemple les clés d'une map `translations`, ou les champs de `pageReactConfig`) ne sont pas restaurées. Configurez celles-ci via l'interface de personnalisation du tableau de bord FastComments à la place.