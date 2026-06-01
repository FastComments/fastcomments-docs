---
Establezca su tenant id una vez en `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Luego agregue una etiqueta donde quiera que aparezca el widget, en un layout, una entrada o una página:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Eso es todo. Reemplace `demo` por su tenant id de FastComments (lo encontrará en [Configuración > API/SSO](https://fastcomments.com/auth/my-account/api)).
---