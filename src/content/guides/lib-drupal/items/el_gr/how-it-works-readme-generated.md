---
Όταν ένας χρήστης επισκέπτεται μια οντότητα με το πεδίο FastComments ενεργοποιημένο:

1. Το widget JavaScript του FastComments φορτώνεται από το CDN.
2. Αν έχει ρυθμιστεί SSO, η ταυτότητα Drupal του χρήστη μεταβιβάζεται στο FastComments.
3. Ένα fallback `<noscript>` παρέχει σχόλια που έχουν αποδοθεί από τον διακομιστή για χρήστες χωρίς JavaScript (μόνο στις λειτουργίες Live Comments και Streaming Chat).
---