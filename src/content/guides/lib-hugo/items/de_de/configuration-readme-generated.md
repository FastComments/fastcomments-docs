Alle FastComments-Widget-Optionen werden unter `[params.fastcomments]` in `hugo.toml` gesetzt und können pro Seite im Front Matter unter `[fastcomments]` überschrieben werden. Reihenfolge der Priorität, von niedrig bis hoch: Site-Parameter, Seiten-Front Matter, Shortcode-Parameter.

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

Wenn weder `url` noch `urlId` angegeben sind, wird `url` standardmäßig auf den Permalink der Seite gesetzt, damit Kommentarthreads an eine stabile URL gebunden bleiben.

### EU data residency

EU-Kunden setzen `region = "eu"`, um das Widget zu `cdn-eu.fastcomments.com` zu routen:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Hinweis zur Groß-/Kleinschreibung von Schlüsseln

Hugo wandelt jeden Schlüssel in `hugo.toml` und im Front Matter in Kleinbuchstaben um, aber die FastComments-Widgets erfordern camelCase-Schlüssel (`tenantId`, `hasDarkBackground`). Diese Komponente stellt die korrekte Groß-/Kleinschreibung für jede bekannte oberste Option automatisch wieder her, schreiben Sie die Optionen also in ihrer normalen camelCase-Form. Schlüssel, die innerhalb eines Objektwerts verschachtelt sind (zum Beispiel die Schlüssel einer `translations`-Map oder Felder von `pageReactConfig`), werden nicht wiederhergestellt. Konfigurieren Sie diese stattdessen über die Anpassungsoberfläche des FastComments-Dashboards.