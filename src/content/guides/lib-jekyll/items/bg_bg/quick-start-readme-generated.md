Задайте tenant id-а си веднъж в `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

След това добавете таг навсякъде, където искате уиджета, в шаблон, публикация или страница:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Това е всичко. Заменете `demo` с вашия FastComments tenant id (намира се в [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api)).