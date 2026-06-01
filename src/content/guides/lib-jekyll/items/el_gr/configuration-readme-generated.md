Config comes from three places. Later sources win:

1. **Παγκόσμιες προεπιλογές** στο `_config.yml` κάτω από το κλειδί `fastcomments:`.
2. **Πλαίσιο σελίδας**, παράγεται αυτόματα για widgets που αφορούν συγκεκριμένη σελίδα (βλέπε παρακάτω).
3. **Ιδιότητες tag** γραμμένες στο ίδιο το tag.

Έτσι ένα `url_id` στο tag υπερισχύει της τιμής που προέρχεται από τη σελίδα, η οποία με τη σειρά της υπερισχύει οποιασδήποτε παγκόσμιας προεπιλογής.

### Σύνταξη χαρακτηριστικών

Οι ιδιότητες είναι ζεύγη `key=value` σε `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Τιμές σε εισαγωγικά** (`"..."` ή `'...'`) είναι κυριολεκτικές συμβολοσειρές.
- **Χωρίς εισαγωγικά** `true`/`false` γίνονται boolean, και οι αριθμοί γίνονται αριθμοί.
- **Χωρίς εισαγωγικά** οτιδήποτε άλλο επιλύεται ως μεταβλητή Liquid από το πλαίσιο της σελίδας, π.χ. `url_id=page.slug`. (Το Liquid δεν επεκτείνει `{% raw %}\{{ ... }}{% endraw %}` μέσα στις ιδιότητες ενός tag, οπότε χρησιμοποιήστε τη μορφή `page.slug` αντί για `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Τα κλειδιά ιδιοτήτων και ρυθμίσεων σε snake_case αντιστοιχίζονται αυτόματα στα camelCase κλειδιά που περιμένει το FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, κ.τ.λ.). Οποιαδήποτε άλλη επιλογή από τη [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) περνάει με τον ίδιο τρόπο.

### Τιμές προερχόμενες από τη σελίδα

Για τα widgets που περιορίζονται σε σελίδα (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) αυτές συμπληρώνονται αυτόματα από την τρέχουσα σελίδα εκτός αν τις ορίσετε εσείς:

- `url_id` ← `page.url` (ένα σταθερό αναγνωριστικό ανεξάρτητο από το επισκεπτόμενο domain)
- `url` ← `site.url` + `page.url` (μόνο όταν το `url` έχει ρυθμιστεί στο `_config.yml`)
- `page_title` ← `page.title`

Τα site-wide widgets (recent comments/discussions, top pages, reviews summary, user activity feed, bulk count) δεν σχετίζονται με κάποια σελίδα και δεν συμπληρώνονται αυτόματα από τέτοιες τιμές.

### Διαμονή δεδομένων στην ΕΕ

Οι πελάτες στην ΕΕ προσθέτουν `region: eu`, είτε παγκοσμίως:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ή ανά tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Τα widgets τότε φορτώνονται από το CDN της ΕΕ.