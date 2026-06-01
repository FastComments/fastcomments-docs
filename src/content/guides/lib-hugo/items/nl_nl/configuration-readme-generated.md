---
Alle FastComments-widgetopties worden ingesteld onder `[params.fastcomments]` in `hugo.toml` en kunnen per pagina worden overschreven in de front matter onder `[fastcomments]`. Voorrang, van laagste naar hoogste: site-parameters, pagina-front-matter, shortcode-parameters.

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

Als noch `url` noch `urlId` is opgegeven, valt `url` terug op de permalink van de pagina zodat reactiedraden aan een stabiele URL blijven gekoppeld.

### EU gegevensresidentie

EU-klanten stellen `region = "eu"` in om de widget naar `cdn-eu.fastcomments.com` te routeren:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Opmerking over hoofdlettergebruik van keys

Hugo zet elke sleutel in `hugo.toml` en de front matter om naar kleine letters, maar de FastComments-widgets vereisen camelCase-sleutels (`tenantId`, `hasDarkBackground`). Deze component herstelt automatisch het correcte hoofdlettergebruik voor elke bekende top-leveloptie, dus schrijf opties in hun normale camelCase-vorm. Sleutels genest binnen een objectwaarde (bijvoorbeeld de sleutels van een `translations`-map, of velden van `pageReactConfig`) worden niet hersteld. Configureer deze in plaats daarvan via de aanpassings-UI van het FastComments-dashboard.
---