[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Για την πιστοποίηση, το FastComments εξαρτάται από το ότι τα cookies τρίτων είναι ενεργοποιημένα στο πρόγραμμα περιήγησής σας. Χωρίς αυτά, οι χρήστες θα πρέπει πάντα να αφήνουν το email τους για να σχολιάσουν (εκτός εάν το πεδίο εισαγωγής email είναι κρυφό), και τα σχόλιά τους θα εμφανίζονται πάντα ως μη επαληθευμένα (από προεπιλογή).

Για να παρακάμψετε αυτό, μπορείτε να ενεργοποιήσετε την παράκαμψη των cookies τρίτων. 

Όταν αυτή η ρύθμιση είναι ενεργοποιημένη, θα εμφανίσει ένα μικρό αναδυόμενο παράθυρο που δείχνει ένα μήνυμα ότι ο χρήστης συνδέεται. Αυτό το αναδυόμενο παράθυρο εμφανίζεται όποτε ο χρήστης αλληλεπιδρά με το widget σχολίων· για παράδειγμα, αν αφήσει ένα σχόλιο.

Μπορούμε να το κάνουμε αυτό με κώδικα ορίζοντας τη σημαία **enableThirdPartyCookieBypass** σε true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Ενεργοποίηση Παράκαμψης Cookies Τρίτων'; code-example-end]

Μπορούμε επίσης να το ρυθμίσουμε μέσω του UI Προσαρμογής Widget, κάτω από `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Σελίδα προσαρμογής widget με το πλαίσιο ελέγχου Enable Third-Party Cookie Popup επιλεγμένο'; title='Ενεργοποίηση Παράκαμψης Cookies Τρίτων' app-screenshot-end]