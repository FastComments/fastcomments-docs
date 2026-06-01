Config stammt aus drei Quellen. Spätere Quellen haben Vorrang:

1. **Globale Standardwerte** in `_config.yml` unter dem `fastcomments:`-Schlüssel.
2. **Seitenkontext**, der automatisch für seitenbezogene Widgets abgeleitet wird (siehe unten).
3. **Tag-Attribute**, die direkt am Tag selbst geschrieben werden.

Ein `url_id` im Tag überschreibt also den seitenabgeleiteten Wert, der wiederum jeden globalen Standard überschreibt.

### Attributsyntax

Attribute sind `key=value`-Paare in `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Quoted** values (`"..."` or `'...'`) are literal strings.
- **Unquoted** `true`/`false` become booleans, and numbers become numbers.
- **Unquoted** anything else is resolved as a Liquid variable from the page context, e.g.
  `url_id=page.slug`. (Liquid does not expand `{% raw %}\{{ ... }}{% endraw %}` inside a tag's
  attributes, so use the bare `page.slug` form rather than `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case-Attribute und Konfigurationsschlüssel werden automatisch auf die camelCase-Schlüssel abgebildet, die FastComments erwartet (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground` usw.). Jede andere Option aus der [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) wird auf dieselbe Weise direkt durchgereicht.

### Von der Seite abgeleitete Werte

Für die seitenbezogenen Widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) werden diese automatisch aus der aktuellen Seite gefüllt, sofern Sie sie nicht selbst setzen:

- `url_id` ← `page.url` (ein stabiler Bezeichner, unabhängig von der besuchenden Domain)
- `url` ← `site.url` + `page.url` (nur wenn `url` in `_config.yml` gesetzt ist)
- `page_title` ← `page.title`

Site-weite Widgets (neueste Kommentare/Diskussionen, Top-Seiten, Bewertungsübersicht, Benutzeraktivitäts-Feed, Sammelzählung) sind nicht an eine Seite gebunden und leiten diese Werte nicht ab.

### EU-Datenresidenz

EU-Kunden fügen `region: eu` hinzu, entweder global:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

oder pro Tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgets werden dann vom EU-CDN geladen.