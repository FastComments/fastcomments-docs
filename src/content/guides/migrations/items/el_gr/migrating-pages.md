Αν και η ονομασία του είναι Εργαλείο Μεταφοράς Domain, μπορεί επίσης να χρησιμοποιηθεί για τη μεταφορά σχολίων μεταξύ μεμονωμένων σελίδων.

Απλώς εισάγετε τις παλιές και νέες τιμές [urlId](/guide-customizations-and-configuration.html#url-id) στα πεδία εισαγωγής `from` και `to`.

### Μεταφορά σελίδων μέσω API

Για να μεταφέρετε πολλές σελίδες ταυτόχρονα, χρησιμοποιήστε το [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Δέχεται έως και 100 ζεύγη `from` και `to` ανά αίτημα και τα εκτελεί ως batch job.

Με το `type` ορισμένο σε `urlId`, μεταφέρεται μόνο η σελίδα της οποίας το `urlId` είναι ακριβώς το `from`, οπότε το `post-1` δεν ταιριάζει με το `post-10`. Το εργαλείο του dashboard δεν διαθέτει αυτή την επιλογή.

Κάθε μεταφορά μέσω API μπορεί να αναιρεθεί για 90 ημέρες με το [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).