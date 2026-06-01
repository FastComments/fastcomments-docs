---
Konfiguracija potiče iz tri izvora. Kasniji izvori imaju prednost:

1. **Globalne podrazumevane vrednosti** u `_config.yml` pod ključem `fastcomments:`.
2. **Kontekst stranice**, automatski izveden za widgete koji su vezani za stranicu (videti dole).
3. **Atributi taga** napisani direktno na tagu.

Dakle, `url_id` na tagu prevazilazi vrednost izvedenu sa stranice, koja pak prevazilazi bilo koju globalnu podrazumevanu vrednost.

### Sintaksa atributa

Atributi su `key=value` parovi u `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Vrednosti u navodnicima** (`"..."` ili `'...'`) su doslovni stringovi.
- **Bez navodnika** `true`/`false` postaju booleani, a brojevi postaju brojevi.
- **Bez navodnika**, sve ostalo se rešava kao Liquid promenljiva iz konteksta stranice, npr. `url_id=page.slug`. (Liquid ne proširuje `{% raw %}\{{ ... }}{% endraw %}` unutar atributa taga, zato koristite oblik `page.slug` umesto `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Ključevi atributa i konfiguracije u snake_case se automatski mapiraju na camelCase ključeve koje FastComments očekuje (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, и т.д.). Bilo koja druga opcija iz [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) prolazi na isti način.

### Vrednosti izvedene iz stranice

Za widgete ograničene na stranicu (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) oni se popunjavaju automatski iz trenutne stranice osim ako ih sami ne postavite:

- `url_id` ← `page.url` (stabilan identifikator nezavistan od domene posetioca)
- `url` ← `site.url` + `page.url` (samo kada je `url` postavljen u `_config.yml`)
- `page_title` ← `page.title`

Widgeti na nivou sajta (recent comments/discussions, top pages, reviews summary, user activity feed, bulk count) nisu vezani za stranicu i ne dobijaju ove vrednosti.

### Rezidencija podataka u EU

EU kupci dodaju `region: eu`, ili globalno:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ili po tagu: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgeti se tada učitavaju sa EU CDN-a.
---