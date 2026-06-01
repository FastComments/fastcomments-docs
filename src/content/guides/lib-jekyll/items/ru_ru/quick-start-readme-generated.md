Задайте ваш tenant id один раз в `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Затем добавьте тег в том месте, где вы хотите разместить виджет — в шаблоне, в посте или на странице:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Готово. Замените `demo` на ваш tenant id FastComments (найти его можно в [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api)).