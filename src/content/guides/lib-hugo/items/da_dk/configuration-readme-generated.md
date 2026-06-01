Alle FastComments-widgetindstillinger indstilles under `[params.fastcomments]` i `hugo.toml`, og kan overskrives per side i front matter under `[fastcomments]`. Prioritet, lavest til højest: site params, page front matter, shortcode-parameter.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Når hverken `url` eller `urlId` er angivet, bruges `url` som standard sidens permalink, så kommentartråde forbliver knyttet til en stabil URL.

### EU data residency

EU-kunder sætter `region = "eu"` for at dirigere widget'en til `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Bemærkning om nøglers store/små bogstaver

Hugo konverterer alle nøgler til små bogstaver i `hugo.toml` og front matter, men FastComments-widgets kræver camelCase-nøgler (`tenantId`, `hasDarkBackground`). Denne komponent gendanner automatisk den korrekte kapitalisering for hver kendt øverste niveau-indstilling, så skriv indstillinger i deres normale camelCase-form. Nøgler indlejret i en objektværdi (for eksempel nøglerne i et `translations`-map, eller felter i `pageReactConfig`) gendannes ikke. Konfigurer disse i stedet via tilpasningsbrugerfladen i FastComments-dashboardet.