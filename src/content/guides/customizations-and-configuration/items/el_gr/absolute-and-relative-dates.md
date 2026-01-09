[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Από προεπιλογή χρησιμοποιούνται τοπικοποιημένες σχετικές ημερομηνίες. Για παράδειγμα, δίπλα σε ένα πρόσφατα δημοσιευμένο σχόλιο μπορεί να δείτε "πριν 11 λεπτά".

Ενδέχεται να χρειαστεί ή να επιθυμείτε να διατηρήσετε αυτήν τη μορφή σχετικής ημερομηνίας, αλλά ταυτόχρονα να εμφανίσετε και την πλήρη ημερομηνία δίπλα της· σε αυτήν την περίπτωση ορίστε αυτήν την παράμετρο σε true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget, κάτω από τις Σύνθετες Επιλογές. Θα πρέπει πρώτα να ενεργοποιήσετε τις Απόλυτες Ημερομηνίες για να δείτε αυτήν την επιλογή στο UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]