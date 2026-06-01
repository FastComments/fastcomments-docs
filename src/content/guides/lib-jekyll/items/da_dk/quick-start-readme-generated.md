Angiv din tenant-id én gang i `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Tilføj derefter et tag, hvor du vil have widgetten, i et layout, et indlæg eller en side:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Det er det. Erstat `demo` med dit FastComments tenant-id (find det under
[Indstillinger > API/SSO](https://fastcomments.com/auth/my-account/api)).