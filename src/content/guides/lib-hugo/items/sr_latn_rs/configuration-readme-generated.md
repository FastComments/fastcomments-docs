Sve FastComments opcije vidžeta su podešene pod `[params.fastcomments]` u `hugo.toml`, i mogu biti prepisane po stranici u front matter-u pod `[fastcomments]`. Precedenca, od najnižeg do najvišeg: parametri sajta, front matter stranice, parametri shortkoda.

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

Kada ni `url` ni `urlId` nisu navedeni, `url` podrazumevano postaje permalink stranice kako bi se konverzacije u komentarima vezale za stabilan URL.

### EU rezidencija podataka

Kupci iz EU postave `region = "eu"` da usmere vidžet na `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Napomena o pisanju ključeva

Hugo pretvara svaki ključ u mala slova u `hugo.toml` i front matter, ali FastComments vidžeti zahtevaju camelCase ključeve (`tenantId`, `hasDarkBackground`). Ovaj modul automatski vraća ispravan oblik pisanja za svaku poznatu top-level opciju, pa opcije pišite u uobičajenom camelCase obliku. Ključevi koji su ugnježdeni unutar objekta (na primer ključevi mape `translations`, ili polja `pageReactConfig`) se ne vraćaju. Umesto toga, njih konfigurišite kroz FastComments kontrolnu tablu za prilagođavanje.