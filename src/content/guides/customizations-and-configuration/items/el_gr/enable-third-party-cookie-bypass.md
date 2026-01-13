[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Για την αυθεντικοποίηση, το FastComments εξαρτάται από τα τρίτα cookies να είναι ενεργοποιημένα στο πρόγραμμα περιήγησής σας. Χωρίς αυτά, οι χρήστες θα πρέπει πάντα να
αφήνουν το email τους για να σχολιάσουν (εκτός αν το πεδίο εισαγωγής email είναι κρυφό), και τα σχόλιά τους θα εμφανίζονται πάντα ως μη επαληθευμένα (από προεπιλογή).

Για να το παρακάμψετε, μπορείτε να ενεργοποιήσετε την παράκαμψη τρίτων cookies. 

Όταν αυτή η ρύθμιση είναι ενεργοποιημένη, προκαλεί ένα μικρό αναδυόμενο παράθυρο που εμφανίζει ένα μήνυμα που λέει ότι ο χρήστης συνδέεται. Αυτό το αναδυόμενο παράθυρο
εμφανίζεται όποτε ο χρήστης αλληλεπιδρά με το widget σχολίων· για παράδειγμα, αν αφήσει ένα σχόλιο.

Μπορούμε να το κάνουμε στον κώδικα ορίζοντας τη σημαία **enableThirdPartyCookieBypass** σε true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Μπορούμε επίσης να το ρυθμίσουμε μέσω του UI Προσαρμογής Widget, στην επιλογή `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---