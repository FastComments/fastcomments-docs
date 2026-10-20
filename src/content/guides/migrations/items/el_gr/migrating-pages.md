While its name is the Domain Migration tool, it can also be used to migrate comments across individual pages.

Απλώς εισάγετε τις παλιές και νέες τιμές [urlId](/guide-customizations-and-configuration.html#url-id) στα πεδία εισόδου `from` και `to`.

### Μεταφορά Σελίδων μέσω API

Για να μεταφέρετε πολλές σελίδες ταυτόχρονα, χρησιμοποιήστε [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Δέχεται έως 100 ζεύγη `from` και `to` ανά αίτηση και τα εκτελεί ως παρτίδα εργασίας.

Με το `type` ορισμένο σε `urlId`, μόνο η σελίδα της οποίας το `urlId` είναι ακριβώς `from` μετακινείται, έτσι το `post-1` δεν ταιριάζει με το `post-10`. Το εργαλείο του πίνακα ελέγχου δεν διαθέτει αυτήν την επιλογή.

Κάθε μεταφορά API μπορεί να αναιρεθεί για 90 ημέρες με [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).