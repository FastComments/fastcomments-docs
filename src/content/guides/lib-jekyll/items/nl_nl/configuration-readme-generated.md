Configuratie komt van drie plaatsen. Latere bronnen hebben voorrang:

1. **Globale standaardwaarden** in `_config.yml` onder de sleutel `fastcomments:`.
2. **Paginacontext**, automatisch afgeleid voor paginagebonden widgets (zie hieronder).
3. **Tag-attributen** geschreven op de tag zelf.

Dus een `url_id` op de tag overschrijft de paginageleide waarde, die een globale standaard overschrijft.

### Attribuutsyntaxis

Attributen zijn `key=value` paren in `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Geciteerde** waarden (`"..."` of `'...'`) zijn letterlijke strings.
- **Niet-geciteerde** `true`/`false` worden booleans, en nummers worden nummers.
- **Niet-geciteerde** alles anders wordt opgelost als een Liquid-variabele uit de paginacontext, bijv.
  `url_id=page.slug`. (Liquid breidt geen `{% raw %}\{{ ... }}{% endraw %}` uit binnen de attributen van een tag, dus gebruik de kale `page.slug`-vorm in plaats van `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case attribuut- en config-sleutels worden automatisch gemapt naar de camelCase-sleutels die FastComments verwacht (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, enzovoort). Elke andere optie uit de
[widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
gaat op dezelfde manier rechtstreeks door.

### Pagina-afgeleide waarden

Voor de paginagebonden widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) worden deze automatisch ingevuld vanaf de
huidige pagina tenzij je ze zelf instelt:

- `url_id` ← `page.url` (een stabiele identificator onafhankelijk van het bezochte domein)
- `url` ← `site.url` + `page.url` (alleen wanneer `url` is ingesteld in `_config.yml`)
- `page_title` ← `page.title`

Site-brede widgets (recente reacties/discussies, top pagina's, samenvatting van beoordelingen, gebruikersactiviteit-feed,
bulk telling) zijn niet aan een pagina gebonden en leiden deze niet af.

### EU-gegevensresidentie

EU-klanten voegen `region: eu` toe, hetzij globaal:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

of per tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgets worden vervolgens geladen vanaf het EU CDN.