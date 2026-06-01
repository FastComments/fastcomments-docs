Konfiguration kommer fra tre steder. Senere kilder har forrang:

1. **Globale standarder** i `_config.yml` under `fastcomments:`-nøglen.
2. **Sidekontekst**, afledt automatisk for sidespecifikke widgets (se nedenfor).
3. **Tag-attributter** skrevet på selve tagget.

Så en `url_id` på tagget overskriver den side-afledte værdi, som overskriver enhver global standard.

### Attribute syntax

Attributter er `key=value` par i `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Anførte** værdier (`"..."` eller `'...'`) er bogstavelige strenge.
- **Uden anførselstegn** `true`/`false` bliver booleanske værdier, og tal bliver tal.
- **Uden anførselstegn** bliver alt andet opløst som en Liquid-variabel fra sidekonteksten, f.eks. `url_id=page.slug`. (Liquid udvider ikke `{% raw %}\{{ ... }}{% endraw %}` inde i et tags attributter, så brug den bare `page.slug`-form i stedet for `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case attribut- og konfigurationsnøgler mappes automatisk til de camelCase-nøgler, FastComments forventer (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, osv.). Enhver anden indstilling fra [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) føres videre på samme måde.

### Værdier afledt fra siden

For de sidespecifikke widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) udfyldes disse automatisk fra den aktuelle side, medmindre du selv sætter dem:

- `url_id` ← `page.url` (en stabil identifikator uafhængig af besøgsdomænet)
- `url` ← `site.url` + `page.url` (kun når `url` er sat i `_config.yml`)
- `page_title` ← `page.title`

Site-dækkende widgets (seneste kommentarer/diskussioner, top-sider, oversigt over anmeldelser, brugeraktivitetsfeed, masseoptælling) er ikke knyttet til en side og afleder ikke disse.

### EU-dataresidens

EU-kunder tilføjer `region: eu`, enten globalt:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

eller per tag: {% raw %}{% fastcomments region="eu" %}{% endraw %}. Widgets indlæses derefter fra EU-CDN'en.