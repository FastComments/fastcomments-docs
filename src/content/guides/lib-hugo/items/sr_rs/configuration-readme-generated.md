Sve FastComments widget opcije se podešavaju pod `[params.fastcomments]` u `hugo.toml`, i mogu se prepisati po strani u front matter pod `[fastcomments]`. Prednost, od najniže do najviše: parametri sajta, front matter stranice, parametri shortcode-a.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter stranice
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Kada nisu navedeni ni `url` ni `urlId`, `url` podrazumevano koristi permalink stranice kako bi niti komentara ostale vezane za stabilan URL.

### EU rezidencija podataka

EU kupci postavljaju `region = "eu"` da usmere widget na `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Napomena o pisanju ključeva (casing)

Hugo pretvara svaki ključ u `hugo.toml` i front matter u mala slova, ali FastComments widgeti zahtevaju camelCase ključeve (`tenantId`, `hasDarkBackground`). Ovaj komponent automatski vraća ispravan zapis velikih/malih slova za svaku poznatu top-level opciju, tako da pišite opcije u njihovom uobičajenom camelCase obliku. Ključevi ugnježdeni unutar vrednosti objekta (na primer ključevi mape `translations`, ili polja `pageReactConfig`) se ne vraćaju. Podesite te kroz kontrolni panel FastComments za prilagođavanje umesto toga.