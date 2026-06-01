Defina seu tenant id uma vez em `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Em seguida, adicione uma tag onde quiser o widget, em um layout, em uma postagem ou em uma página:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Pronto. Substitua `demo` pelo seu tenant id do FastComments (encontre-o em [Configurações > API/SSO](https://fastcomments.com/auth/my-account/api)).