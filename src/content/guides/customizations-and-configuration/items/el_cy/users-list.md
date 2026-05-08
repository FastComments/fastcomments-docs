[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Κατά προεπιλογή, το FastComments δεν εμφανίζει λίστα χρηστών στη σελίδα.

Μπορείτε να εμφανίσετε μια λίστα ατόμων που αυτή τη στιγμή βλέπουν τη σελίδα, δίπλα στο widget σχολίων. Η λίστα ενημερώνεται ζωντανά καθώς οι χρήστες μπαίνουν και φεύγουν, και δείχνει το όνομά τους, το avatar και μια ένδειξη ότι είναι συνδεδεμένοι.

Υπάρχουν τρεις επιλογές διάταξης:

- `1` - Top: μια οριζόντια σειρά επικαλυπτόμενων avatars που εμφανίζεται πάνω από τα σχόλια.
- `2` - Left: μια πλαϊνή στήλη με ονόματα και κουκκίδες online που εμφανίζεται στα αριστερά του widget.
- `3` - Right: η ίδια πλαϊνή στήλη που εμφανίζεται στα δεξιά του widget.

Ορίστε τη σημαία **usersListLocation** για να ενεργοποιήσετε τη λειτουργία:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Κατά προεπιλογή, η λίστα δείχνει μόνο χρήστες που είναι αυτή τη στιγμή online. Για να συμπεριλάβετε επίσης άτομα που έχουν σχολιάσει στη σελίδα στο παρελθόν (αλλά δεν την βλέπουν αυτή τη στιγμή), ορίστε **usersListIncludeOffline** σε true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Οι προηγούμενοι σχολιαστές εμφανίζονται χωρίς την πράσινη κουκκίδα σύνδεσης, ώστε να είναι σαφές ποιοι είναι παρόντες αυτή τη στιγμή.

Οι χρήστες με ιδιωτικά προφίλ εμφανίζονται με γενικό avatar και ετικέτα "Ιδιωτικό Προφίλ", ώστε ο αριθμός να παραμένει ακριβής χωρίς να αποκαλύπτονται ταυτότητες.

Αυτό μπορεί επίσης να ρυθμιστεί χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Θέση λίστας χρηστών". Όταν η θέση οριστεί σε οτιδήποτε άλλο εκτός από "Απενεργοποιημένο", εμφανίζεται από κάτω ένα πλαίσιο επιλογής "Συμπερίληψη προηγούμενων σχολιαστών".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]