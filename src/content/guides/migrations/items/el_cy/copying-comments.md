Σε περίπτωση που χρειάζεται να μετακινηθούν δεδομένα, το FastComments παρέχει ένα εργαλείο αυτοεξυπηρέτησης για τη μεταφορά σχολίων μεταξύ σελίδων και άρθρων.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Συμπλήρωση των "Από" Πεδίων

Για να αποφασίσουμε από πού θα μετακινηθούν τα σχόλια, χρειαζόμαστε απλώς το `URL ID` προέλευσης.

If you aren't passing a value for `urlId` in the comment widget configuration, then this will be a "clean" version of the page URL.

Μπορείτε να δείτε ποιες τιμές έχουν τα σχόλιά σας για το `URL ID` εξάγοντάς τα.

### Συμπλήρωση των "Προς" Πεδίων

Για να αποφασίσουμε προς τα πού θα μετακινηθούν τα σχόλια, χρειαζόμαστε το στοχευόμενο `URL ID` και το `URL`.

Το `URL ID` θα είναι ο κάδος στον οποίο θα καταχωρηθεί το σχόλιο. Το πεδίο `URL` χρησιμοποιείται ώστε να μπορείτε να μεταβείτε απευθείας στο σχόλιο από e‑mails και τα εργαλεία διαχείρισης/επιτήρησης.

#### WordPress

Αν χρησιμοποιείτε WordPress, για παράδειγμα, θα εισάγετε τα IDs των άρθρων στα πεδία `URL ID` των επιλογών Προς/Από στο εργαλείο μετανάστευσης, αντί για ένα URL.