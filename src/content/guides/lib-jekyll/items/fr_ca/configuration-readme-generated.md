Config vient de trois endroits. Les sources ultérieures prévalent :

1. **Valeurs par défaut globales** dans `_config.yml` sous la clé `fastcomments:`.
2. **Contexte de la page**, dérivé automatiquement pour les widgets limités à la page (voir ci‑dessous).
3. **Attributs du tag** écrits sur la balise elle‑même.

Ainsi, un `url_id` sur la balise remplace la valeur dérivée de la page, qui remplace toute valeur par défaut globale.

### Attribute syntax

Les attributs sont des paires `key=value` en `snake_case` :

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Les valeurs entre guillemets** (`"..."` ou `'...'`) sont des chaînes littérales.
- **Les valeurs non entre guillemets** `true`/`false` deviennent des booléens, et les nombres deviennent des nombres.
- **Tout autre élément non entre guillemets** est résolu en tant que variable Liquid provenant du contexte de la page, p.ex. `url_id=page.slug`. (Liquid n'expande pas `{% raw %}\{{ ... }}{% endraw %}` à l'intérieur des attributs d'une balise, donc utilisez la forme brute `page.slug` plutôt que `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Les clés d'attributs et de configuration en snake_case sont automatiquement mappées vers les clés en camelCase attendues par FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, et ainsi de suite). Toute autre option de la [configuration du widget](https://docs.fastcomments.com/guide-customizations-and-configuration.html) est transmise de la même manière.

### Page-derived values

Pour les widgets limités à la page (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) ceux-ci sont remplis automatiquement à partir de la page courante, à moins que vous ne les définissiez vous‑même :

- `url_id` ← `page.url` (un identifiant stable indépendant du domaine visité)
- `url` ← `site.url` + `page.url` (uniquement lorsque `url` est défini dans `_config.yml`)
- `page_title` ← `page.title`

Les widgets globaux (commentaires/discussions récents, pages principales, résumé des avis, flux d'activité des utilisateurs, comptage groupé) ne sont pas liés à une page et ne déduisent pas ces valeurs.

### EU data residency

Les clients de l'UE ajoutent `region: eu`, soit globalement :

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ou par balise : `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Les widgets seront alors chargés depuis le CDN de l'UE.