[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Τα σχόλια μπορούν να κλειδωθούν ώστε να μην μπορούν να προστεθούν νέα σχόλια ή ψήφοι ορίζοντας τη σημαία readonly σε true.

Τα σχόλια επίσης δεν θα μπορούν να επεξεργαστούν ή να διαγραφούν.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget, για ολόκληρο domain, ή για σελίδα:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Ενημέρωση!

Από τον Νοέμβριο του 2022, τα threads μπορούν να κλειδωθούν ή να ξεκλειδωθούν **σε πραγματικό χρόνο** από διαχειριστές και συντονιστές μέσω του μενού με τις τρεις τελείες πάνω από την περιοχή απάντησης.

Αυτό θα αποτρέπει νέα σχόλια, ενώ ταυτόχρονα επιτρέπει την ψήφιση και δίνει τη δυνατότητα στους χρήστες να διαγράψουν τα σχόλιά τους εάν το επιθυμούν, ενώ το `readonly` δεν επιτρέπει αυτά.

Αυτό αντιστοιχεί στο πεδίο `isClosed` στο API `Page`.