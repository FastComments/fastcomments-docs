---
Définissez votre tenant id une seule fois dans `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Ensuite, ajoutez une balise là où vous voulez le widget, dans un layout, un article ou une page:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

C'est tout. Remplacez `demo` par votre FastComments tenant id (vous le trouverez sous
[Paramètres > API/SSO](https://fastcomments.com/auth/my-account/api)).
---