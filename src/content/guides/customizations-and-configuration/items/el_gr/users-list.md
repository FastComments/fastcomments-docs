[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments δεν εμφανίζει λίστα χρηστών στη σελίδα.

Μπορείτε να εμφανίσετε μια λίστα με άτομα που αυτήν τη στιγμή βλέπουν τη σελίδα, δίπλα στο widget σχολίων. Η λίστα ενημερώνεται σε πραγματικό χρόνο καθώς οι χρήστες εισέρχονται και φεύγουν, και δείχνει το όνομά τους, το avatar τους και έναν δείκτη "σε σύνδεση".

Υπάρχουν τρεις επιλογές διάταξης:

- `1` - Πάνω: μια οριζόντια σειρά επικαλυπτόμενων avatar που εμφανίζεται πάνω από τα σχόλια.
- `2` - Αριστερά: μια πλαϊνή στήλη με ονόματα και κουκκίδες σύνδεσης που εμφανίζεται στα αριστερά του widget.
- `3` - Δεξιά: η ίδια πλαϊνή στήλη που εμφανίζεται στα δεξιά του widget.

Ορίστε τη σημαία **usersListLocation** για να ενεργοποιήσετε τη λειτουργία:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Από προεπιλογή η λίστα εμφανίζει μόνο χρήστες που είναι αυτήν τη στιγμή συνδεδεμένοι. Για να συμπεριλάβετε επίσης άτομα που έχουν σχολιάσει στη σελίδα στο παρελθόν (αλλά δεν την προβάλλουν τώρα), ορίστε **usersListIncludeOffline** σε true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Οι προηγούμενοι σχολιαστές εμφανίζονται χωρίς την πράσινη κουκκίδα "σε σύνδεση" ώστε να είναι σαφές ποιοι είναι παρόντες αυτήν τη στιγμή.

Οι χρήστες με ιδιωτικά προφίλ εμφανίζονται με ένα γενικό avatar και την ετικέτα "Ιδιωτικό Προφίλ", ώστε ο αριθμός να παραμένει ακριβής χωρίς να αποκαλύπτονται οι ταυτότητες.

Αυτό μπορεί επίσης να ρυθμιστεί χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Θέση Λίστας Χρηστών". Όταν η θέση οριστεί σε οτιδήποτε διαφορετικό από Απενεργοποιημένο, εμφανίζεται από κάτω ένα πλαίσιο επιλογής "Συμπερίληψη προηγούμενων σχολιαστών".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Για πάνω από 500 ενεργούς χρήστες, η λίστα μπορεί να έχει καθυστέρηση έως 30 δευτερόλεπτα.