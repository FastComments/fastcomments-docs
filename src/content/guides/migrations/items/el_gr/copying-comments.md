Σε περίπτωση που χρειαστεί να μεταφερθούν δεδομένα, το FastComments παρέχει ένα εργαλείο αυτοεξυπηρέτησης για τη μετακίνηση σχολίων
μεταξύ σελίδων και άρθρων.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Συμπλήρωση των πεδίων «Από»

Για να αποφασίσουμε από πού θα μετακινηθούν τα σχόλια, χρειαζόμαστε απλώς το αναγνωριστικό προέλευσης `URL ID`.

If you aren't passing a value for `urlId` in the comment widget configuration, then this will be a "clean" version of the page URL.

You can see what values your comments have for `URL ID` by exporting them.

### Συμπλήρωση των πεδίων «Προς»

Για να αποφασίσουμε σε ποιο σημείο θα μετακινηθούν τα σχόλια, πρέπει να γνωρίζουμε το προοριζόμενο `URL ID` και το `URL`.

Το `URL ID` θα είναι ο «κάδος» στον οποίο θα μπει το σχόλιο. Το `URL` πεδίο χρησιμοποιείται ώστε να μπορείτε να πλοηγηθείτε απευθείας
στο σχόλιο από email και από εργαλεία διαχείρισης.

#### WordPress

Εάν χρησιμοποιείτε το WordPress, για παράδειγμα θα εισάγετε τα αναγνωριστικά άρθρων στα πεδία Προς/Από `URL ID` στο εργαλείο μετανάστευσης,
αντί για ένα URL.