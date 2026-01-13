[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα έχει ενεργοποιημένα τα ζωντανά σχόλια.

Αυτό σημαίνει ότι κάθε θεατής του νήματος σχολίων θα βλέπει το ίδιο περιεχόμενο.

Για παράδειγμα, αν προστεθεί ένα σχόλιο, αυτό το σχόλιο θα εμφανίζεται. Εάν ένα σχόλιο επεξεργαστεί ή διαγραφεί,
τότε τα σχόλια αυτά θα επεξεργαστούν ή θα διαγραφούν για όλους τους θεατές του νήματος. Το ίδιο ισχύει για τις ψήφους και για όλες τις ενέργειες διαχείρισης.

Ωστόσο, μπορούμε να απενεργοποιήσουμε αυτό:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα «Απενεργοποίηση ζωντανών σχολίων».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---