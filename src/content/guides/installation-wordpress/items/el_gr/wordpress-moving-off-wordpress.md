Αν μεταφέρετε τον ιστότοπό σας από το WordPress και θέλετε το FastComments στον νέο ιστότοπο, δεν χρειάζεστε το πρόσθετο WordPress. Εξάγετε τα σχόλιά σας από το WordPress, στη συνέχεια ανεβάστε το αρχείο στη [Import page](https://fastcomments.com/auth/my-account/manage-data/import) στον πίνακα ελέγχου του FastComments.

Υποστηρίζουμε δύο μορφές εξαγωγής WordPress.

### WordPress XML (Συνιστάται)

Αυτό είναι το αρχείο από τον ενσωματωμένο εξαγωγέα του WordPress, οπότε δεν χρειάζεται επιπλέον πρόσθετο.

1. Στο διαχειριστικό του WordPress, μεταβείτε στο `Tools -> Export`.
2. Επιλέξτε `All content` και κάντε κλικ στο `Download Export File`.
3. Στη σελίδα FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), επιλέξτε `WordPress (.xml)` και ανεβάστε το αρχείο.

Κάθε σχόλιο συνδέεται με το URL της ανάρτησης στην οποία έγινε, το οποίο περιλαμβάνεται ήδη στο αρχείο.

Η εισαγωγή διατηρεί το όνομα του συγγραφέα, το email και τον ιστότοπο, την ημερομηνία, το περιεχόμενο, τη ιεραρχία απαντήσεων και αν το σχόλιο εγκρίθηκε. Τα avatar των σχολιαστών μεταφέρονται από το Gravatar. Οι ψήφοι δεν περιλαμβάνονται σε αυτή τη μορφή.

### WordPress CSV

Αυτό είναι το αρχείο από το [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Εγκαταστήστε το πρόσθετο στο διαχειριστικό του WordPress και εξάγετε τα σχόλιά σας ως CSV.
2. Αντικαταστήστε κάθε τιμή `comment_post_ID` με το URL της ανάρτησης.
3. Στη σελίδα FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), επιλέξτε `WordPress (.csv)` και ανεβάστε το αρχείο.

Κάθε σχόλιο συνδέεται με τη στήλη `comment_post_ID`. Το WordPress γεμίζει αυτή τη στήλη με το ID της ανάρτησης, και ο νέος σας ιστότοπος δεν διαθέτει IDs WordPress, έτσι το βήμα 2 το αντικαθιστά με το URL.

Η εισαγωγή διατηρεί το όνομα του συγγραφέα, το email και τον ιστότοπο, την ημερομηνία, το περιεχόμενο, τη ιεραρχία απαντήσεων και αν το σχόλιο εγκρίθηκε. Τα avatar των σχολιαστών μεταφέρονται από το Gravatar. Διατηρεί επίσης τη σημαία spam του WordPress και τα likes και dislikes του wpDiscuz όταν το αρχείο τα περιλαμβάνει.

### Αντιστοίχιση Σχολίων στις Νέες Σελίδες Σας

Αν ο νέος σας ιστότοπος διατηρεί τα ίδια URLs με τον ιστότοπο WordPress, τα σχόλια εμφανίζονται στις αντίστοιχες σελίδες χωρίς επιπλέον ρύθμιση.

Αν αλλάξει το domain, εκτελέστε το [Domain Migration tool](/guide-migrations.html#migrating-domains) μετά την εισαγωγή. Αν αλλάξουν τα URLs μεμονωμένων σελίδων, μπορείτε να [migrate each page](/guide-migrations.html#migrating-pages) από το παλιό URL στο νέο.

Για μαζικές μετα迁σεις σελίδων, όπως η αφαίρεση του domain από την τιμή που περνάτε στο πεδίο [urlId](/guide-customizations-and-configuration.html#url-id) του widget σχολίων, [open a support ticket](https://fastcomments.com/auth/my-account/help) και θα το διαχειριστούμε για εσάς.

### Πριν Αλλάξετε

Μπορείτε να εκτελέσετε την εισαγωγή όσες φορές θέλετε. Η επανεισαγωγή του ίδιου αρχείου [does not create duplicates](/guide-migrations.html#importing-data), έτσι μπορείτε να εισάγετε μία φορά για να δοκιμάσετε τον νέο ιστότοπο, έπειτα να εισάγετε ξανά με τα πιο πρόσφατα σχόλια ακριβώς πριν την αλλαγή.

Για αρχεία εξαγωγής μεγαλύτερα από 1 GB, [reach out to support](https://fastcomments.com/auth/my-account/help).

Για να προσθέσετε το FastComments στον νέο σας ιστότοπο, δείτε τον [Installation guide](/guide-installation.html).