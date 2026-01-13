[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

Το FastComments έχει σχεδιαστεί ώστε να προσαρμόζεται, και η γραμματοσειρά που χρησιμοποιούν τα widget μας δεν αποτελεί εξαίρεση.

Από προεπιλογή, το FastComments χρησιμοποιεί το `system font stack` για να εμφανίζεται όσο το δυνατόν καλύτερα σε ένα ευρύ φάσμα συσκευών.

Για να ορίσετε τις δικές σας γραμματοσειρές, δείτε την [Τεκμηρίωση Custom CSS](/guide-customizations-and-configuration.html#custom-css).

Εκεί θα βρείτε έναν τρόπο να ορίσετε προσαρμοσμένο CSS, που θα σας επιτρέψει να καθορίσετε τις επιθυμητές γραμματοσειρές.

#### Πώς να Ορίσετε τη Γραμματοσειρά

Για να παρακάμψετε τη γραμματοσειρά, προτείνουμε να ορίσετε το CSS σας χρησιμοποιώντας τους selectors `.fast-comments, textarea`. Για παράδειγμα:

[inline-code-attrs-start title = 'Παράδειγμα Εξωτερικής Προσαρμοσμένης Γραμματοσειράς'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---