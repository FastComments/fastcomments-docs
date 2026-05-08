[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Κατά προεπιλογή, το FastComments δεν εμφανίζει λίστα χρηστών στη σελίδα.

Μπορείτε να εμφανίσετε μια λίστα ατόμων που αυτή τη στιγμή βλέπουν τη σελίδα, δίπλα στο widget των σχολίων. Η λίστα ενημερώνεται σε πραγματικό χρόνο καθώς οι χρήστες εισέρχονται και φεύγουν, και εμφανίζει το όνομά τους, το avatar και έναν δείκτη online.

Υπάρχουν τρεις επιλογές διάταξης:

- `1` - Top: μια οριζόντια σειρά επικαλυπτόμενων avatar που εμφανίζεται πάνω από τα σχόλια.
- `2` - Left: μια πλαϊνή στήλη με ονόματα και ενδείξεις online που εμφανίζεται αριστερά του widget.
- `3` - Right: η ίδια πλαϊνή στήλη που εμφανίζεται δεξιά του widget.

Ορίστε τη σημαία **usersListLocation** για να ενεργοποιήσετε τη λειτουργία:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Κατά προεπιλογή η λίστα δείχνει μόνο τους χρήστες που είναι αυτή τη στιγμή online. Για να συμπεριλάβετε επίσης άτομα που έχουν σχολιάσει τη σελίδα στο παρελθόν (αλλά δεν τη βλέπουν τώρα), ορίστε **usersListIncludeOffline** σε true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Οι προηγούμενοι σχολιαστές εμφανίζονται χωρίς την πράσινη ένδειξη online, ώστε να είναι σαφές ποιοι είναι παρόντες αυτή τη στιγμή.

Χρήστες με ιδιωτικά προφίλ εμφανίζονται με ένα γενικό avatar και μια ετικέτα "Private Profile" έτσι ώστε ο αριθμός να παραμένει ακριβής χωρίς να αποκαλύπτονται ταυτότητες.

Αυτό μπορεί επίσης να διαμορφωθεί χωρίς κώδικα. Στη σελίδα προσαρμογής widget, δείτε την επιλογή "Users List Location":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Όταν η τοποθεσία οριστεί σε οτιδήποτε άλλο εκτός του Off, εμφανίζεται από κάτω το πλαίσιο επιλογής "Include past commenters":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]