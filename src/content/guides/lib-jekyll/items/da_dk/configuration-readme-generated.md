Konfiguration kommer fra tre steder. Senere kilder vinder:

1. **Globale standardindstillinger** i `_config.yml` under nøglen `fastcomments:`.
2. **Sidekontekst**, som udledes automatisk for side-omfattede widgets (se nedenfor).
3. **Tag-attributter**, som skrives direkte på tagget.

Så en `url_id` på tagget tilsidesætter den sideudledte værdi, som igen tilsidesætter enhver global standardindstilling.

### Attributsyntaks

Attributter er `key=value`-par i `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Anført i citationstegn** (`"..."` eller `'...'`) behandles som bogstavelige strenge.
- **Uden citationstegn** bliver `true`/`false` til booleans, og tal bliver til tal.
- **Uden citationstegn** løses alt andet som en Liquid-variabel fra sidekonteksten, f.eks.
  `url_id=page.slug`. (Liquid udvider ikke `{% raw %}\{{ ... }}{% endraw %}` inde i et tags
  attributter, så brug den rene `page.slug`-form frem for `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case-attribut- og konfigurationsnøgler mappes automatisk til de camelCase-nøgler, FastComments
forventer (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground` og så videre). Enhver anden indstilling fra
[widget-konfigurationen](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
sendes direkte igennem på samme måde.

### Sideudledte værdier

For de side-omfattede widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) udfyldes disse automatisk fra den
aktuelle side, medmindre du selv angiver dem:

- `url_id` ← `page.url` (en stabil identifikator uafhængig af det besøgende domæne)
- `url` ← `site.url` + `page.url` (kun når `url` er angivet i `_config.yml`)
- `page_title` ← `page.title`

Sideoverdækkende widgets (seneste kommentarer/diskussioner, topssider, anmeldelsesoversigt,
brugeraktivitetsfeed, bulk-antal) er ikke knyttet til en side og udleder ikke disse værdier.

### EU-dataresidens

EU-kunder tilføjer `region: eu`, enten globalt:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

eller pr. tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgets indlæses derefter fra EU-CDN.