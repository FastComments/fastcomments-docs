[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments δεν εμφανίζει λίστα χρηστών στη σελίδα.

Μπορείτε να εμφανίσετε λίστα με τους ανθρώπους που αυτή τη στιγμή βλέπουν τη σελίδα, δίπλα στο widget σχολίων. Η λίστα ενημερώνεται σε πραγματικό χρόνο καθώς οι χρήστες μπαίνουν και φεύγουν, και δείχνει το όνομά τους, το avatar και έναν δείκτη online.

Υπάρχουν τρεις επιλογές διάταξης:

- `1` - Επάνω: μια οριζόντια σειρά επικαλυπτόμενων avatars που απεικονίζονται πάνω από τα σχόλια.
- `2` - Αριστερά: μια πλαϊνή στήλη με ονόματα και κουκκίδες online που απεικονίζεται στα αριστερά του widget.
- `3` - Δεξιά: η ίδια πλαϊνή στήλη που απεικονίζεται στα δεξιά του widget.

Ορίστε τη σημαία **usersListLocation** για να ενεργοποιήσετε τη λειτουργία:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Από προεπιλογή, η λίστα δείχνει μόνο χρήστες που είναι αυτή τη στιγμή online. Για να συμπεριλάβετε επίσης άτομα που έχουν σχολιάσει τη σελίδα στο παρελθόν (αλλά δεν την βλέπουν τώρα), ορίστε **usersListIncludeOffline** σε true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Οι προηγούμενοι σχολιαστές εμφανίζονται χωρίς την πράσινη κουκκίδα online, ώστε να είναι σαφές ποιος είναι παρών αυτή τη στιγμή.

Οι χρήστες με ιδιωτικά προφίλ εμφανίζονται με ένα γενικό avatar και μία ετικέτα «Ιδιωτικό Προφίλ» ώστε ο αριθμός να παραμένει ακριβής χωρίς να αποκαλύπτονται ταυτότητες.

Αυτό μπορεί επίσης να ρυθμιστεί χωρίς κώδικα. Στη σελίδα προσαρμογής widget, δείτε την επιλογή "Τοποθεσία Λίστας Χρηστών". Όταν η τοποθεσία οριστεί σε οτιδήποτε άλλο εκτός από Off, εμφανίζεται από κάτω ένα πλαίσιο επιλογής "Συμπερίληψη προηγούμενων σχολιαστών".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Για πάνω από 500 ενεργούς χρήστες, η λίστα μπορεί να έχει καθυστέρηση έως 30 δευτερόλεπτα.