---
Ορίστε το tenant ID σας μία φορά στο `hugo.toml`:

```toml
[params.fastcomments]
  tenantId = "demo"   # αντικαταστήστε το "demo" με το tenant ID του FastComments σας
```

Στη συνέχεια, είτε ενσωματώστε το widget σχολίων στο θέμα σας (δείτε [Ενσωμάτωση Θέματος](#theme-integration-readme-generated)), είτε εισάγετε ένα shortcode στο Markdown οποιασδήποτε σελίδας:

```text
\{{< fastcomments >}}
```
---