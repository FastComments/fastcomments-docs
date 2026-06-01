---
Ορίστε το tenant id σας μία φορά στο `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Στη συνέχεια, προσθέστε μια ετικέτα όπου θέλετε το widget, σε ένα layout, ένα post ή μια σελίδα:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Αυτό είναι όλο. Αντικαταστήστε το `demo` με το tenant id σας στο FastComments (βρείτε το κάτω από
[Ρυθμίσεις > API/SSO](https://fastcomments.com/auth/my-account/api)).
---