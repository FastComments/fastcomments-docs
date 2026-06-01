Définissez l'ID de votre locataire une fois dans `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Ensuite, ajoutez une balise où vous voulez le widget, dans un layout, un article ou une page:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

C'est tout. Remplacez `demo` par l'ID de votre locataire FastComments (vous le trouverez dans [Paramètres > API/SSO](https://fastcomments.com/auth/my-account/api)).