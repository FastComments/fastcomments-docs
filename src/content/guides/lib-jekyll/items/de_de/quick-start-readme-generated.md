Legen Sie Ihre Tenant-ID einmal in `_config.yml` fest:

```yaml
fastcomments:
  tenant_id: demo
```

Fügen Sie dann ein Tag an der Stelle ein, an der das Widget erscheinen soll, in einem Layout, einem Beitrag oder auf einer Seite:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Das war's. Ersetzen Sie `demo` durch Ihre FastComments Tenant-ID (finden Sie diese unter [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).