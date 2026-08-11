[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα ζητά μόνο το σχόλιο του χρήστη, το όνομα χρήστη και το email του.

Ωστόσο, σε ορισμένες περιπτώσεις μπορεί να θέλετε ο χρήστης να αφήσει έναν σύνδεσμο προς το δικό του blog ή ιστότοπο.

Μπορούμε να ενεργοποιήσουμε την εμφάνιση ενός επιπλέον πεδίου εισαγωγής για τη διεύθυνση URL του ιστότοπου του χρήστη ορίζοντας τη σημαία **enableCommenterLinks** σε true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Ενεργοποίηση Συνδέσμων Σχολιαστών'; code-example-end]

Όταν παρέχεται η εν λόγω URL, ο λογαριασμός του χρήστη θα ενημερωθεί και όλα τα ονόματα χρήστη του σε όλα τα παλιά και μελλοντικά σχόλια θα συνδέονται με αυτή τη διεύθυνση URL.

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Σελίδα προσαρμογής widget με το πλαίσιο ελέγχου συνδέσμων σχολιαστών επιλεγμένο για την προσθήκη πεδίου URL ιστότοπου στη φόρμα σχολίου'; title='Ενεργοποίηση Συνδέσμων Σχολιαστών' app-screenshot-end]