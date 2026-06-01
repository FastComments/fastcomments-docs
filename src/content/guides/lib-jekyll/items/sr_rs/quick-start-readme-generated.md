---
Подесите свој tenant id једном у `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Затим додајте таг где год желите видгет, у изгледу, посту или на страници:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

То је све. Замените `demo` вашим FastComments tenant id-ом (пронађите га под
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).
---