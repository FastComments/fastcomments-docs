Подесите ваш tenant id једном у `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Затим додајте таг гдје год желите видџет, у шаблону, посту или на страници:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

То је то. Замјените `demo` својим FastComments tenant id-ом (нађите га под
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).