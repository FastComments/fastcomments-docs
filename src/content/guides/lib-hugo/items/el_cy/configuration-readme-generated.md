---
Όλες οι επιλογές του widget FastComments ορίζονται κάτω από το `[params.fastcomments]` στο `hugo.toml`, και μπορούν να παρακαμφθούν ανά σελίδα στο front matter κάτω από το `[fastcomments]`. Προτεραιότητα, από τη χαμηλότερη στη υψηλότερη: παράμετροι του site, front matter της σελίδας, παράμετροι του shortcode.

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

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### Τοποθεσία δεδομένων στην ΕΕ

Οι πελάτες στην ΕΕ ορίζουν `region = "eu"` για να δρομολογήσουν το widget στο `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Σημείωση σχετικά με τη μορφοποίηση (case) των κλειδιών

Hugo μετατρέπει όλα τα κλειδιά σε πεζά στο `hugo.toml` και στο front matter, αλλά τα widgets του FastComments απαιτούν κλειδιά σε camelCase (`tenantId`, `hasDarkBackground`). Αυτό το συστατικό αποκαθιστά αυτόματα τη σωστή μορφοποίηση (casing) για κάθε γνωστή κορυφαία επιλογή, οπότε γράφετε τις επιλογές στη συνηθισμένη camelCase μορφή τους. Κλειδιά που είναι εμφωλευμένα μέσα σε μια τιμή αντικειμένου (για παράδειγμα τα κλειδιά ενός χάρτη `translations`, ή πεδία του `pageReactConfig`) δεν αποκαθίστανται. Ρυθμίστε αυτά μέσω του UI προσαρμογής στο dashboard του FastComments.
---