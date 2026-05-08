[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments δεν εμφανίζει λίστα χρηστών στη σελίδα.

Μπορείτε να εμφανίσετε μια λίστα ατόμων που αυτή τη στιγμή βλέπουν τη σελίδα, δίπλα στο widget σχολίων. Η λίστα ενημερώνεται σε πραγματικό χρόνο καθώς οι χρήστες μπαίνουν και φεύγουν, και δείχνει το όνομά τους, το avatar και έναν δείκτη online.

Υπάρχουν τρεις επιλογές διάταξης:

- `1` - Επάνω: μια οριζόντια σειρά επικαλυπτόμενων avatar που αποδίδεται πάνω από τα σχόλια.
- `2` - Αριστερά: μία πλαϊνή στήλη με ονόματα και τελείες online που αποδίδεται στα αριστερά του widget.
- `3` - Δεξιά: η ίδια πλαϊνή στήλη που αποδίδεται στα δεξιά του widget.

Ορίστε τη σημαία **usersListLocation** για να ενεργοποιήσετε τη λειτουργία:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Εμφάνιση λίστας χρηστών στα δεξιά'; code-example-end]

Από προεπιλογή, η λίστα δείχνει μόνο τους χρήστες που είναι αυτήν τη στιγμή online. Για να συμπεριλάβετε επίσης άτομα που έχουν σχολιάσει στη σελίδα στο παρελθόν (αλλά δεν την βλέπουν τώρα), ορίστε **usersListIncludeOffline** σε true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Συμπερίληψη προηγούμενων σχολιαστών'; code-example-end]

Οι προηγούμενοι σχολιαστές αποδίδονται χωρίς την πράσινη τελεία online, ώστε να είναι σαφές ποιοι είναι παρόντες αυτήν τη στιγμή.

Οι χρήστες με ιδιωτικά προφίλ εμφανίζονται με ένα γενικό avatar και μια ετικέτα "Ιδιωτικό προφίλ", ώστε ο αριθμός να παραμένει ακριβής χωρίς να αποκαλύπτονται ταυτότητες.

Αυτό μπορεί επίσης να ρυθμιστεί χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Θέση λίστας χρηστών". Όταν η θέση οριστεί σε οτιδήποτε άλλο εκτός της επιλογής "Απενεργοποίηση", εμφανίζεται από κάτω ένα πλαίσιο επιλογής "Συμπερίληψη προηγούμενων σχολιαστών".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Ρυθμίσεις λίστας χρηστών'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---