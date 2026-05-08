[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Κατά προεπιλογή, το FastComments δεν εμφανίζει λίστα χρηστών στη σελίδα.

Μπορείτε να εμφανίσετε μια λίστα με τα άτομα που αυτή τη στιγμή βλέπουν τη σελίδα, δίπλα στο widget σχολίων. Η λίστα ενημερώνεται σε πραγματικό χρόνο καθώς οι χρήστες μπαίνουν και φεύγουν, και εμφανίζει το όνομά τους, το avatar και έναν δείκτη online.

Υπάρχουν τρεις επιλογές διάταξης:

- `1` - Top: μια οριζόντια σειρά επικαλυπτόμενων avatars που αποδίδεται πάνω από τα σχόλια.
- `2` - Left: μια πλαϊνή στήλη με ονόματα και κουκκίδες online που αποδίδεται στα αριστερά του widget.
- `3` - Right: η ίδια πλαϊνή στήλη που αποδίδεται στα δεξιά του widget.

Ορίστε τη σημαία **usersListLocation** για να ενεργοποιήσετε τη λειτουργία:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Κατά προεπιλογή η λίστα εμφανίζει μόνο χρήστες που είναι αυτή τη στιγμή online. Για να συμπεριλάβετε επίσης άτομα που σχολίασαν στη σελίδα στο παρελθόν (αλλά δεν την βλέπουν αυτή τη στιγμή), ορίστε **usersListIncludeOffline** σε true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Οι παλαιότεροι σχολιαστές εμφανίζονται χωρίς την πράσινη κουκκίδα online ώστε να είναι σαφές ποιοι είναι παρόντες αυτή τη στιγμή.

Οι χρήστες με ιδιωτικά προφίλ εμφανίζονται με ένα γενικό avatar και μια ετικέτα "Private Profile" ώστε ο αριθμός να παραμένει ακριβής χωρίς να αποκαλύπτονται ταυτότητες.

Αυτό μπορεί επίσης να ρυθμιστεί χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Όταν η θέση οριστεί σε οτιδήποτε άλλο εκτός από Off, εμφανίζεται από κάτω το πλαίσιο επιλογής "Include past commenters":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]

---