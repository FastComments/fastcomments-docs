Vse možnosti vtičnika FastComments so nastavljene pod `[params.fastcomments]` v `hugo.toml` in jih je mogoče preglasiti za vsako stran posebej v front matter pod `[fastcomments]`. Prednost, od najnižje do najvišje: parametri spletnega mesta, front matter strani, parametri shortcode.

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

Ko ni podanega niti `url` niti `urlId`, se `url` privzeto nastavi na permalink strani, tako da so nitke komentarjev vezane na stabilen URL.

### Rezidenca podatkov v EU

Stranke v EU nastavijo `region = "eu"` za usmeritev vtičnika na `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Opomba o velikosti črk ključev

Hugo pretvori vse ključe v `hugo.toml` in front matter v male črke, vendar FastComments vtičniki zahtevajo ključe v camelCase (`tenantId`, `hasDarkBackground`). Ta komponenta samodejno obnovi pravilno zapisovanje črk za vse znane vrhnje možnosti, zato vnesite možnosti v njihovi običajni camelCase obliki. Ključi, vgrajeni znotraj vrednosti objekta (na primer ključi v mapi `translations` ali polja `pageReactConfig`), niso obnovljeni. Namesto tega jih konfigurirajte preko uporabniškega vmesnika za prilagajanje na nadzorni plošči FastComments.