[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, οι απαντήσεις στα σχόλια κορυφαίου επιπέδου εμφανίζονται.

Αυτό μπορεί να ρυθμιστεί έτσι ώστε ο χρήστης να πρέπει να κάνει κλικ "Show Replies" στα σχόλια κορυφαίου επιπέδου για να δει τα παιδιά.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Αυτή η ρύθμιση δεν θα επηρεάσει τον αριθμό των αρχικά φορτωμένων σχολίων κορυφαίου επιπέδου. Εάν έχετε ένα σχόλιο κορυφαίου επιπέδου και 29 απαντήσεις, με αυτή τη ρύθμιση ενεργοποιημένη θα:

- Θα δείτε το σχόλιο κορυφαίου επιπέδου.
- Θα δείτε Show Replies (29) κάτω από αυτό το σχόλιο.

Εάν επιθυμείτε να εμφανίζονται όλα τα σχόλια κορυφαίου επιπέδου σε συνδυασμό με αυτήν την επιλογή, ορίστε [starting page to -1](#starting-page).