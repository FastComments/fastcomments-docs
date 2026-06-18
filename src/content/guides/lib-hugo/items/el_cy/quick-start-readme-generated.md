---
Ρυθμίστε το tenant ID σας μία φορά στο `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # αντικαταστήστε "demo" με το αναγνωριστικό tenant του FastComments
```

Στη συνέχεια είτε ενσωματώστε το widget σχολίων στο θέμα σας (δείτε [Ενσωμάτωση Θέματος](#theme-integration-readme-generated)), είτε εισάγετε ένα shortcode στο Markdown οποιασδήποτε σελίδας:

```text
\{{< fastcomments >}}
```
---