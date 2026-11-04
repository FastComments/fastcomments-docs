Όλες οι επιλογές του widget FastComments ορίζονται κάτω από `[params.fastcomments]` στο `hugo.toml` και μπορούν να παρακαμφθούν ανά σελίδα στο front matter κάτω από `[fastcomments]`. Σειρά προτεραιότητας, από τη χαμηλότερη στην υψηλότερη: παράμετροι site, front matter σελίδας, παράμετροι shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Όταν δεν παρέχεται ούτε `url` ούτε `urlId`, το `url` ορίζεται από προεπιλογή στο permalink της σελίδας, ώστε τα νήματα σχολίων να παραμένουν συνδεδεμένα με ένα σταθερό URL.

### Διαμονή δεδομένων στην ΕΕ

Οι πελάτες ΕΕ ορίζουν `region = "eu"` για να δρομολογείται το widget στο `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Σημείωση για την πεζογραφία των κλειδιών

Το Hugo μετατρέπει όλα τα κλειδιά στο `hugo.toml` και στο front matter σε πεζά, αλλά τα widgets του FastComments απαιτούν κλειδιά σε camelCase (`tenantId`, `hasDarkBackground`). Αυτό το component επαναφέρει αυτόματα τη σωστή πεζογραφία για κάθε γνωστή επιλογή κορυφαίου επιπέδου, οπότε γράφετε τις επιλογές στην κανονική τους μορφή camelCase. Τα κλειδιά που είναι εμφωλευμένα μέσα σε τιμή αντικειμένου (για παράδειγμα τα κλειδιά ενός map `translations` ή τα πεδία του `pageReactConfig`) δεν επαναφέρονται. Ρυθμίστε αυτά μέσω του UI προσαρμογής του dashboard του FastComments.