Die Konfiguration stammt aus drei Quellen. Spätere Quellen haben Vorrang:

1. **Globale Standardwerte** in `_config.yml` unter dem Schlüssel `fastcomments:`.
2. **Seitenkontext**, der automatisch für seitenspezifische Widgets abgeleitet wird (siehe unten).
3. **Tag-Attribute**, die direkt am Tag geschrieben werden.

Ein `url_id` am Tag überschreibt also den seitenspezifisch abgeleiteten Wert, welcher wiederum jeden globalen Standardwert überschreibt.

### Attribut-Syntax

Attribute sind `key=value`-Paare in `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **In Anführungszeichen gesetzte** Werte (`"..."` oder `'...'`) sind literale Zeichenketten.
- **Ohne Anführungszeichen** werden `true`/`false` zu Booleans und Zahlen zu Zahlen.
- **Ohne Anführungszeichen** wird alles andere als Liquid-Variable aus dem Seitenkontext aufgelöst, z. B.
  `url_id=page.slug`. (Liquid expandiert `{% raw %}\{{ ... }}{% endraw %}` nicht innerhalb der
  Attribute eines Tags, daher verwenden Sie die nackte Form `page.slug` statt `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Snake_case-Attribut- und Konfigurationsschlüssel werden automatisch auf die camelCase-Schlüssel gemappt, die
FastComments erwartet (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground` usw.). Jede andere Option aus der
[Widget-Konfiguration](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
wird auf dieselbe Weise direkt durchgereicht.

### Seitenspezifisch abgeleitete Werte

Für die seitenspezifischen Widgets (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) werden diese automatisch aus der
aktuellen Seite gefüllt, sofern Sie sie nicht selbst setzen:

- `url_id` ← `page.url` (ein stabiler Bezeichner, unabhängig von der besuchten Domain)
- `url` ← `site.url` + `page.url` (nur wenn `url` in `_config.yml` gesetzt ist)
- `page_title` ← `page.title`

Seitenweite Widgets (neueste Kommentare/Diskussionen, Top-Seiten, Bewertungsübersicht, Benutzeraktivitäts-Feed,
Bulk-Count) sind nicht an eine Seite gebunden und leiten diese Werte nicht ab.

### EU-Datenresidenz

EU-Kunden fügen `region: eu` hinzu, entweder global:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

oder pro Tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgets werden dann über das EU-CDN geladen.