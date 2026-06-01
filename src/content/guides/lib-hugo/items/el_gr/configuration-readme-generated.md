---
Όλες οι επιλογές του widget FastComments ορίζονται κάτω από `[params.fastcomments]` στο `hugo.toml`, και μπορούν να παρακαμφθούν ανά σελίδα στο front matter κάτω από `[fastcomments]`. Προτεραιότητα, από τη χαμηλότερη στη υψηλότερη: παράμετροι ιστότοπου, front matter της σελίδας, παράμετροι shortcode.

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

### Διαμονή δεδομένων στην ΕΕ

Οι πελάτες στην ΕΕ ορίζουν `region = "eu"` για να δρομολογήσουν το widget στο `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Σημείωση σχετικά με τη χρήση πεζών/κεφαλαίων στα κλειδιά

Το Hugo μετατρέπει κάθε κλειδί σε πεζά στο `hugo.toml` και στο front matter, αλλά τα widgets FastComments απαιτούν κλειδιά σε camelCase (`tenantId`, `hasDarkBackground`). Αυτό το component αποκαθιστά αυτόματα τη σωστή χρήση πεζών/κεφαλαίων για κάθε γνωστή κορυφαίου επιπέδου επιλογή, οπότε γράψτε τις επιλογές στη συνήθη μορφή camelCase. Τα κλειδιά που είναι φωλιασμένα μέσα σε μια τιμή αντικειμένου (για παράδειγμα τα κλειδιά ενός χάρτη `translations`, ή πεδία του `pageReactConfig`) δεν αποκαθίστανται. Διαμορφώστε αυτά μέσω του UI προσαρμογής του πίνακα ελέγχου FastComments.
---