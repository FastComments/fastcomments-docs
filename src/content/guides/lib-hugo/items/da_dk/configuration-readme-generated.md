Alle FastComments widget-indstillinger sættes under `[params.fastcomments]` i `hugo.toml`, og kan overskrives pr. side i front matter under `[fastcomments]`. Prioritet, fra lavest til højst: site-parametre, sidens front matter, shortcode-parametre.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# en sides front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Når hverken `url` eller `urlId` er angivet, falder `url` tilbage til sidens permalink, så kommentartråde forbliver knyttet til en stabil URL.

### EU-dataresidens

EU-kunder sætter `region = "eu"` for at dirigere widgeten til `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Bemærkning om nøglecasing

Hugo konverterer alle nøgler i `hugo.toml` og front matter til små bogstaver, men FastComments-widgets kræver camelCase-nøgler (`tenantId`, `hasDarkBackground`). Denne komponent gendanner automatisk den korrekte casing for alle kendte topniveau-indstillinger, så du kan skrive indstillingerne i deres normale camelCase-form. Nøgler, der er indlejret i en objektværdi (for eksempel nøglerne i et `translations`-map eller felterne i `pageReactConfig`), gendannes ikke. Konfigurér disse via FastComments dashboardets tilpasnings-UI i stedet.