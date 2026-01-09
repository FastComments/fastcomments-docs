[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Από προεπιλογή χρησιμοποιούνται τοπικοποιημένες σχετικές ημερομηνίες. Για παράδειγμα, δίπλα σε ένα πρόσφατα δημοσιευμένο σχόλιο μπορεί να δείτε "11 λεπτά πριν".

Μπορεί να είναι αναγκαίο ή επιθυμητό να διατηρηθεί αυτή η σχετική μορφή ημερομηνίας, αλλά επίσης να εμφανίζεται και η πλήρης ημερομηνία παράλληλα — σε αυτή την περίπτωση ορίστε αυτή την παράμετρο σε true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget, στην ενότητα Advanced Options. Πρώτα θα πρέπει να ενεργοποιήσετε τις Απόλυτες Ημερομηνίες για να δείτε αυτή την επιλογή στο UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]