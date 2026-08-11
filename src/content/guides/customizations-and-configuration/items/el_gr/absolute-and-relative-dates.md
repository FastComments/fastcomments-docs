[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, χρησιμοποιούνται τοπικές σχετικές ημερομηνίες. Για παράδειγμα, δίπλα σε ένα πρόσφατα αφήσμένο σχόλιο μπορεί να δείτε "11 λεπτά πριν".

Μπορεί να είναι απαραίτητο ή επιθυμητό να διατηρηθεί αυτή η σχετική μορφή ημερομηνίας, αλλά επίσης να εμφανίζεται η πλήρης ημερομηνία δίπλα της, οπότε ορίζετε αυτήν τη παράμετρο σε true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget, στην ενότητα Προηγμένες Επιλογές. Θα πρέπει πρώτα να ενεργοποιήσετε τις Απόλυτες Ημερομηνίες για να δείτε αυτήν την επιλογή στη διεπαφή χρήστη.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Προηγμένες Επιλογές στη σελίδα προσαρμογής του widget με ενεργοποιημένες τόσο τις απόλυτες ημερομηνίες όσο και τη συνδυασμένη ρύθμιση σχετικής ημερομηνίας'; title='Χρήση και των Απόλυτων και των Σχετικών Ημερομηνιών' app-screenshot-end]