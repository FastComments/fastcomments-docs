Config provient de trois endroits. Les sources ultérieures l'emportent :

1. **Valeurs par défaut globales** dans `_config.yml` sous la clé `fastcomments:`.
2. **Contexte de la page**, dérivé automatiquement pour les widgets à portée de page (voir ci‑dessous).
3. **Attributs du tag** écrits directement sur le tag.

Ainsi, un `url_id` sur le tag remplace la valeur dérivée de la page, qui remplace toute valeur par défaut globale.

### Attribute syntax

Les attributs sont des paires `key=value` en `snake_case` :

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- Les valeurs **entre guillemets** (`"..."` ou `'...'`) sont des chaînes littérales.
- Les valeurs **non entre guillemets** `true`/`false` deviennent des booléens, et les nombres deviennent des nombres.
- Tout ce qui est **non entre guillemets** et autre est résolu comme une variable Liquid à partir du contexte de la page, par ex.
  `url_id=page.slug`. (Liquid n'expande pas `{% raw %}\{{ ... }}{% endraw %}` à l'intérieur des attributs d'un tag, utilisez donc la forme bare `page.slug` plutôt que `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Les clés d'attribut et de configuration en snake_case sont automatiquement mappées aux clés camelCase attendues par FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, etc.). Toute autre option provenant de la
[configuration du widget](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
est transmise telle quelle de la même manière.

### Page-derived values

Pour les widgets à portée de page (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`), ceux-ci sont remplis automatiquement à partir de la
page courante sauf si vous les définissez vous‑même :

- `url_id` ← `page.url` (un identifiant stable indépendant du domaine visité)
- `url` ← `site.url` + `page.url` (uniquement lorsque `url` est défini dans `_config.yml`)
- `page_title` ← `page.title`

Les widgets à l'échelle du site (commentaires/discussions récents, pages les plus consultées, synthèse des avis, fil d'activité des utilisateurs,
compte en masse) ne sont pas liés à une page et ne dérivent pas ces valeurs.

### EU data residency

Les clients de l'UE ajoutent `region: eu`, soit globalement :

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ou par tag : `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Les widgets se chargent alors depuis le CDN de l'UE.