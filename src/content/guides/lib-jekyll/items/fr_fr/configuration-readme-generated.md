La configuration provient de trois sources. Les sources ultérieures ont priorité :

1. **Valeurs par défaut globales** dans `_config.yml` sous la clé `fastcomments:`.
2. **Contexte de la page**, dérivé automatiquement pour les widgets liés à la page (voir ci-dessous).
3. **Attributs de balise** écrits sur la balise elle-même.

Ainsi, un `url_id` sur la balise remplace la valeur dérivée de la page, qui remplace toute valeur par défaut globale.

### Syntaxe des attributs

Les attributs sont des paires `key=value` en `snake_case` :

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- Les valeurs **entre guillemets** (`"..."` ou `'...'`) sont des chaînes littérales.
- Les valeurs **non quotées** `true`/`false` deviennent des booléens, et les nombres deviennent des nombres.
- Toute autre valeur **non quotée** est résolue comme une variable Liquid à partir du contexte de la page, par ex.
  `url_id=page.slug`. (Liquid n'interprète pas `{% raw %}\{{ ... }}{% endraw %}` dans les attributs d'une balise,
  utilisez donc la forme `page.slug` plutôt que `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Les clés d'attributs et de configuration en snake_case sont automatiquement mappées aux clés camelCase attendues par FastComments
(`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, etc.). Toute autre option de la
[configuration du widget](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
passe directement de la même manière.

### Valeurs dérivées de la page

Pour les widgets liés à la page (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`), ces valeurs sont remplies automatiquement à partir de la
page courante, sauf si vous les définissez vous-même :

- `url_id` ← `page.url` (un identifiant stable indépendant du domaine visité)
- `url` ← `site.url` + `page.url` (uniquement si `url` est défini dans `_config.yml`)
- `page_title` ← `page.title`

Les widgets à l'échelle du site (commentaires/discussions récents, pages populaires, résumé des avis, flux d'activité utilisateur,
comptage en masse) ne sont pas liés à une page et ne dérivent pas ces valeurs.

### Résidence des données dans l'UE

Les clients de l'UE ajoutent `region: eu`, soit globalement :

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

soit par balise : `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Les widgets se chargent alors depuis le CDN de l'UE.