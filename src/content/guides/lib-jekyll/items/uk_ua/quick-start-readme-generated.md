Встановіть ваш tenant id один раз у `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Потім додайте тег там, де ви хочете розмістити віджет — у шаблоні, у дописі або на сторінці:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Ось і все. Замініть `demo` на ваш FastComments tenant id (знайдіть його в [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).