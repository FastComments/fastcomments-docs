[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Κατά προεπιλογή, το FastComments θα ζητά μόνο από τον χρήστη το σχόλιό του, το όνομα χρήστη και το email του.

Ωστόσο, σε ορισμένες περιπτώσεις μπορεί να θέλετε ο χρήστης να αφήνει έναν σύνδεσμο προς το δικό του blog ή ιστοσελίδα.

Μπορούμε να ενεργοποιήσουμε την εμφάνιση ενός επιπλέον πεδίου εισαγωγής για να αφήσει ο χρήστης το URL της ιστοσελίδας του ρυθμίζοντας τη σημαία **enableCommenterLinks** σε true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Όταν δοθεί αυτό το URL, ο λογαριασμός του χρήστη θα ενημερωθεί και όλα τα ονόματα χρήστη του σε όλα τα προηγούμενα και μελλοντικά σχόλια θα συνδέονται με αυτό το URL.

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---