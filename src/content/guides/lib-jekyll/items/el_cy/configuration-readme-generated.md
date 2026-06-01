Η διαμόρφωση προέρχεται από τρία μέρη. Οι νεότερες πηγές υπερισχύουν:

1. **Παγκόσμιες προεπιλογές** στο `_config.yml` κάτω από το κλειδί `fastcomments:`.
2. **Πλαίσιο σελίδας**, που προκύπτει αυτόματα για widgets με εμβέλεια σελίδας (βλέπε παρακάτω).
3. **Χαρακτηριστικά tag** γραμμένα στο ίδιο το tag.

Έτσι ένα `url_id` στο tag υπερισχύει της τιμής που προέρχεται από τη σελίδα, η οποία υπερισχύει οποιασδήποτε παγκόσμιας προεπιλογής.

### Σύνταξη χαρακτηριστικών

Τα χαρακτηριστικά είναι ζεύγη `key=value` σε `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Οι τιμές με εισαγωγικά** (`"..."` ή `'...'`) είναι λεκτικές συμβολοσειρές.
- **Χωρίς εισαγωγικά** `true`/`false` γίνονται boolean, και αριθμοί γίνονται αριθμοί.
- **Χωρίς εισαγωγικά** οτιδήποτε άλλο επιλύεται ως μεταβλητή Liquid από το πλαίσιο της σελίδας, π.χ. `url_id=page.slug`. (Liquid δεν επεκτείνει `{% raw %}\{{ ... }}{% endraw %}` μέσα στα attributes ενός tag, οπότε χρησιμοποιήστε τη γυμνή μορφή `page.slug` αντί για `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Τα κλειδιά χαρακτηριστικών σε snake_case και τα κλειδιά ρύθμισης αντιστοιχίζονται αυτόματα στα camelCase κλειδιά που περιμένει το FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, κ.λπ.). Οποιαδήποτε άλλη επιλογή από τη [διαμόρφωση widget](https://docs.fastcomments.com/guide-customizations-and-configuration.html) περνάει απευθείας με τον ίδιο τρόπο.

### Τιμές προερχόμενες από τη σελίδα

Για τα widgets με εμβέλεια σελίδας (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) αυτά συμπληρώνονται αυτόματα από την τρέχουσα σελίδα εκτός κι αν τα ορίσετε εσείς:

- `url_id` ← `page.url` (σταθερό αναγνωριστικό ανεξάρτητο από το domain που το επισκέπτεται)
- `url` ← `site.url` + `page.url` (μόνο όταν έχει οριστεί `url` στο `_config.yml`)
- `page_title` ← `page.title`

Τα site-wide widgets (πρόσφατα σχόλια/συζητήσεις, κορυφαίες σελίδες, περίληψη κριτικών, ροή δραστηριότητας χρηστών, μαζικός μετρητής) δεν συσχετίζονται με μια σελίδα και δεν προέρχονται από αυτές τις τιμές.

### Φιλοξενία δεδομένων στην ΕΕ

Οι πελάτες στην ΕΕ προσθέτουν `region: eu`, είτε παγκοσμίως:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ή ανά tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Τα widgets τότε φορτώνονται από το CDN της ΕΕ.