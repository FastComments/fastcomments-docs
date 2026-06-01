---
Sve FastComments widget opcije postavljaju se pod `[params.fastcomments]` u `hugo.toml`, i mogu se nadjačati po stranici u front matteru pod `[fastcomments]`. Prioritet, od najnižeg do najvišeg: site parametri, front matter stranice, parametri shortcode-a.

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

Kada nije naveden ni `url` ni `urlId`, `url` se zadano postavlja na permalink stranice tako da su niti komentara vezane uz stabilan URL.

### EU smještaj podataka

Kupci iz EU postave `region = "eu"` kako bi usmjerili widget na `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Napomena o veličini slova u ključevima

Hugo pretvara svaki ključ u mala slova u `hugo.toml` i front matteru, ali FastComments widgeti zahtijevaju camelCase ključeve (`tenantId`, `hasDarkBackground`). Ova komponenta automatski vraća ispravno pisanje velikih i malih slova za svaku poznatu vrhunsku opciju, pa upisujte opcije u njihovom uobičajenom camelCase obliku. Ključevi ugniježđeni unutar vrijednosti objekta (na primjer ključevi mape `translations`, ili polja `pageReactConfig`) se ne obnavljaju. Umjesto toga, konfigurirajte ih kroz FastComments sučelje za prilagodbu (dashboard).
---