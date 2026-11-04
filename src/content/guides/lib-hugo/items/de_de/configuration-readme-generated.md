Alle FastComments-Widget-Optionen werden unter `[params.fastcomments]` in `hugo.toml` gesetzt und können pro Seite im Front Matter unter `[fastcomments]` überschrieben werden. Rangfolge, von niedrig nach hoch: Site-Parameter, Front Matter der Seite, Shortcode-Parameter.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# Front Matter einer Seite
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Wenn weder `url` noch `urlId` angegeben ist, wird `url` standardmäßig auf den Permalink der Seite gesetzt, damit Kommentar-Threads an eine stabile URL gebunden bleiben.

### EU-Datenresidenz

EU-Kunden setzen `region = "eu"`, um das Widget an `cdn-eu.fastcomments.com` zu leiten:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Hinweis zur Schreibweise der Schlüssel

Hugo wandelt jeden Schlüssel in `hugo.toml` und im Front Matter in Kleinschreibung um, aber die FastComments-Widgets benötigen camelCase-Schlüssel (`tenantId`, `hasDarkBackground`). Diese Komponente stellt die korrekte Schreibweise für jede bekannte Option auf oberster Ebene automatisch wieder her, sodass Optionen in ihrer normalen camelCase-Form geschrieben werden können. Schlüssel, die innerhalb eines Objektwerts verschachtelt sind (z. B. die Schlüssel einer `translations`-Map oder Felder von `pageReactConfig`), werden nicht wiederhergestellt. Konfigurieren Sie diese stattdessen über die Anpassungs-UI des FastComments-Dashboards.