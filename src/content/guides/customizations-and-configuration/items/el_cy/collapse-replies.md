[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, οι απαντήσεις στα σχόλια ανώτατου επιπέδου εμφανίζονται.

Αυτό μπορεί να ρυθμιστεί έτσι ώστε ο χρήστης να πρέπει να κλικάρει "Εμφάνιση Απαντήσεων" στα σχόλια ανώτατου επιπέδου για να δει τα υποσχόλια.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Αυτή η ρύθμιση δεν θα επηρεάσει τον αριθμό των αρχικά φορτωμένων σχολίων ανώτατου επιπέδου. Αν έχετε ένα σχόλιο ανώτατου επιπέδου και 29 υποσχόλια, με αυτή τη ρύθμιση ενεργοποιημένη θα:

- Θα δείτε το σχόλιο ανώτατου επιπέδου.
- Θα δείτε "Εμφάνιση Απαντήσεων (29)" κάτω από αυτό το σχόλιο.

Εάν θέλετε να εμφανίζετε όλα τα σχόλια ανώτατου επιπέδου σε συνδυασμό με αυτή την επιλογή, ορίστε [την αρχική σελίδα σε -1](#starting-page).