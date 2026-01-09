[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα έχει ενεργοποιημένα τα ζωντανά σχόλια.

Αυτό σημαίνει ότι κάθε θεατής του νήματος σχολίων θα πρέπει να βλέπει το ίδιο περιεχόμενο.

Για παράδειγμα, εάν προστεθεί ένα σχόλιο, αυτό το σχόλιο θα εμφανιστεί. Εάν ένα σχόλιο επεξεργαστεί ή αφαιρεθεί,
τότε αυτά τα σχόλια θα επεξεργαστούν ή θα αφαιρεθούν για όλους τους θεατές του νήματος. Το ίδιο ισχύει για τις ψήφους και για όλες τις ενέργειες εποπτείας.

Ωστόσο, μπορούμε να το απενεργοποιήσουμε:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Απενεργοποίηση ζωντανών σχολίων".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]