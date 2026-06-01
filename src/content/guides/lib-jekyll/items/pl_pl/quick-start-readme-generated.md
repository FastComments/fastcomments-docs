---
Ustaw swój tenant id raz w `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Następnie dodaj tag wszędzie tam, gdzie chcesz mieć widżet — w szablonie, we wpisie lub na stronie:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

To wszystko. Zastąp `demo` swoim tenant id FastComments (znajdziesz go w [Ustawienia > API/SSO](https://fastcomments.com/auth/my-account/api)).
---