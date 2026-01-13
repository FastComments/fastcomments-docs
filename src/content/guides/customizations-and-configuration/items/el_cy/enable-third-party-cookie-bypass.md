[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Για τον έλεγχο ταυτότητας, το FastComments εξαρτάται από τα cookies τρίτων να είναι ενεργοποιημένα στον περιηγητή σας. Χωρίς αυτά, οι χρήστες θα πρέπει πάντα να
αφήνουν το e-mail τους για να σχολιάσουν (εκτός αν το πεδίο εισαγωγής e-mail είναι κρυφό), και τα σχόλιά τους θα εμφανίζονται πάντα ως μη επαληθευμένα (από προεπιλογή).

Για να το παρακάμψετε, μπορείτε να ενεργοποιήσετε την παράκαμψη των cookies τρίτων. 

Όταν αυτή η ρύθμιση είναι ενεργοποιημένη, θα εμφανίζεται ένα μικρό αναδυόμενο παράθυρο που δείχνει ένα μήνυμα ότι ο χρήστης συνδέεται. Αυτό το αναδυόμενο
εμφανίζεται κάθε φορά που ο χρήστης αλληλεπιδρά με το widget σχολίων· για παράδειγμα, αν αφήσει ένα σχόλιο.

Μπορούμε να το κάνουμε στον κώδικα θέτοντας τη σημαία **enableThirdPartyCookieBypass** σε true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Μπορούμε επίσης να το ρυθμίσουμε μέσω της Διεπαφής Προσαρμογής Widget, κάτω από `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---