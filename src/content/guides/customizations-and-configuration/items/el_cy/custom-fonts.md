[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments is designed to be customized, and the font our widgets use is no exception.

By default, FastComments uses the `system font stack` to look as good as possible on a wide range of devices.

To define your own fonts, see the [Τεκμηρίωση προσαρμοσμένου CSS](/guide-customizations-and-configuration.html#custom-css).

Εκεί θα βρείτε έναν τρόπο να ορίσετε προσαρμοσμένο CSS, το οποίο θα σας επιτρέψει να καθορίσετε τις γραμματοσειρές που επιθυμείτε.

#### Πώς να Ορίσετε τη Γραμματοσειρά

To override the font, we recommend you define your CSS using the `.fast-comments, textarea` selectors. For example:

[inline-code-attrs-start title = 'Παράδειγμα Εξωτερικής Προσαρμοσμένης Γραμματοσειράς'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---