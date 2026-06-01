Konfiguracija dolazi iz tri mjesta. Kasniji izvori imaju prednost:

1. **Globalne zadane postavke** u `_config.yml` pod ključem `fastcomments:`.
2. **Kontekst stranice**, izveden automatski za widgete ograničene na stranicu (pogledajte dolje).
3. **Atributi taga** napisani na samom tagu.

Dakle, `url_id` na tagu nadjačava vrijednost dobivenu sa stranice, koja nadjačava bilo koju globalnu zadanu vrijednost.

### Sintaksa atributa

Atributi su parovi `key=value` u `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **U navodnicima** vrijednosti (`"..."` ili `'...'`) su doslovni stringovi.
- **Bez navodnika** `true`/`false` postaju logičke vrijednosti, a brojevi postaju numeričke vrijednosti.
- **Bez navodnika** sve ostalo se rješava kao Liquid varijabla iz konteksta stranice, npr. `url_id=page.slug`. (Liquid ne proširuje `{% raw %}\{{ ... }}{% endraw %}` unutar atributa taga, stoga koristite oblik `page.slug` umjesto `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Ključevi atributa i konfiguracije u snake_case automatski se preslikavaju u camelCase ključeve koje FastComments očekuje (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, i tako dalje). Bilo koja druga opcija iz [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) prolazi bez promjena na isti način.

### Vrijednosti dobivene sa stranice

Za widgete ograničene na stranicu (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) ove se vrijednosti ispunjavaju automatski iz trenutne stranice osim ako ih sami ne postavite:

- `url_id` ← `page.url` (stabilni identifikator neovisan o domeni posjetitelja)
- `url` ← `site.url` + `page.url` (samo kada je `url` postavljen u `_config.yml`)
- `page_title` ← `page.title`

Widgeti koji su globalni za stranicu (najnoviji komentari/rasprave, top stranice, sažetak recenzija, feed aktivnosti korisnika, grupno brojanje) nisu vezani uz stranicu i ne nasljeđuju ove vrijednosti.

### EU rezidencija podataka

EU kupci dodaju `region: eu`, ili globalno:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ili po tagu: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgeti se tada učitavaju s EU CDN-a.