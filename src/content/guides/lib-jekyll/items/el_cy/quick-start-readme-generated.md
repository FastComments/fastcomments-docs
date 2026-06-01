---
Ορίστε το tenant id σας μία φορά στο `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Στη συνέχεια, προσθέστε ένα tag όπουδήποτε θέλετε το widget, σε ένα layout, ένα post ή μια σελίδα:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Αυτό ήταν. Αντικαταστήστε το `demo` με το tenant id του FastComments σας (θα το βρείτε κάτω από
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).
---