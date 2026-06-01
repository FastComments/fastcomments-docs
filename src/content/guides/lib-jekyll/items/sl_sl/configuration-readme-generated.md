Konfiguracija izvira iz treh virov. Novejši viri imajo prednost:

1. **Globalne privzete vrednosti** v `_config.yml` pod ključem `fastcomments:`.
2. **Kontekst strani**, samodejno izpeljan za gradnike, vezane na stran (glej spodaj).
3. **Atributi oznake**, zapisani neposredno na oznaki.

Torej `url_id` na oznaki preglasi vrednost, izpeljano iz strani, ta pa preglasi katero koli globalno privzeto vrednost.

### Sintaksa atributov

Atributi so pari `key=value` v `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **V narekovajih** vrednosti (`"..."` ali `'...'`) so dobesedni nizi.
- **Brez narekovajev** `true`/`false` postaneta booleana, številke pa številke.
- **Brez narekovajev** vse ostalo se razreši kot Liquid spremenljivka iz konteksta strani, npr. `url_id=page.slug`. (Liquid ne razširi `{% raw %}\{{ ... }}{% endraw %}` znotraj atributov oznake, zato uporabite golo obliko `page.slug` namesto `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Ključi atributov in konfiguracije v snake_case se samodejno preslikajo v camelCase ključe, ki jih FastComments pričakuje (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, in tako naprej). Katere koli druge možnosti iz [konfiguracija gradnika](https://docs.fastcomments.com/guide-customizations-and-configuration.html) se prenesejo na enak način.

### Vrednosti izpeljane iz strani

Za gradnike, vezane na stran (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) so te vrednosti samodejno izpolnjene iz trenutne strani, razen če jih sami nastavite:

- `url_id` ← `page.url` (stabilen identifikator, neodvisen od domene obiskovalca)
- `url` ← `site.url` + `page.url` (samo ko je `url` nastavljen v `_config.yml`)
- `page_title` ← `page.title`

Gradniki na ravni spletnega mesta (najnovejši komentarji/razprave, najbolj obiskane strani, povzetek ocen, vir uporabniške aktivnosti, množično štetje) niso vezani na posamezno stran in teh vrednosti ne izpeljejo.

### Shranjevanje podatkov v EU

Stranke v EU dodajo `region: eu`, bodisi globalno:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ali za posamezno oznako: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Gradniki se nato naložijo iz EU CDN.