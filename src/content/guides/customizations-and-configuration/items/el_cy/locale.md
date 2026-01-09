[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Από προεπιλογή, το FastComments θα αποδώσει το widget σχολιασμού στην locale που καθορίζεται από το σύστημα και τον περιηγητή του χρήστη.

Όταν ένας χρήστης σχολιάζει ή συνδέεται, ενημερώνουμε την τελευταία locale που χρησιμοποίησε και τη χρησιμοποιούμε επίσης για την αποστολή email.

Αυτό επηρεάζει τον τρόπο με τον οποίο μεταφράζεται το widget σχολιασμού για τον χρήστη. Η locale αποτελείται από τη γλώσσα και την περιοχή του χρήστη, οπότε η ρύθμιση της locale συνήθως θα αλλάξει τη γλώσσα που χρησιμοποιείται για να εμφανίζεται το κείμενο στον χρήστη.

#### Μέσω του UI

Αυτό μπορεί να οριστεί χρησιμοποιώντας τη διεπαφή προσαρμογής του widget. Δείτε την επιλογή "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Μέσω κώδικα

Αυτό μπορεί να παρακαμφθεί με την επιθυμητή locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Υποστηριζόμενες γλώσσες και κωδικοί locale

[Μπορείτε να βρείτε την πλήρη λίστα των υποστηριζόμενων γλωσσών και των αντίστοιχων κωδικών locale εδώ.](/guide-supported-languages.html#supported-languages)

### Σημείωση SSO

Εάν χρησιμοποιείτε SSO, ίσως θελήσετε να περάσετε την locale του χρήστη στο user object, ώστε τα email και άλλα στοιχεία να είναι σωστά τοπικοποιημένα για τον χρήστη.

---