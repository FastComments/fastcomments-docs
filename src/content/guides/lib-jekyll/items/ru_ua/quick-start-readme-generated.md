---
Укажите ваш tenant id один раз в `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Затем добавьте тег туда, где вы хотите разместить виджет — в layout, в посте или на странице:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Готово. Замените `demo` на ваш FastComments tenant id (найдите его в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).
---