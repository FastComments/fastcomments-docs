[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα έχει ενεργοποιημένα τα ζωντανά σχόλια.

Αυτό σημαίνει ότι κάθε θεατής της αλυσίδας σχολίων πρέπει να βλέπει το ίδιο περιεχόμενο.

Για παράδειγμα, εάν προστεθεί ένα σχόλιο, αυτό το σχόλιο πρέπει να εμφανιστεί. Εάν ένα σχόλιο επεξεργαστεί ή αφαιρεθεί,
τότε αυτά τα σχόλια θα επεξεργαστούν ή αφαιρεθούν για όλους τους θεατές της αλυσίδας. Το ίδιο ισχύει για τις ψήφους και όλες τις ενέργειες διαχείρισης.

Ωστόσο, μπορούμε να το απενεργοποιήσουμε:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Τμήμα \"Disable Live Commenting\" της σελίδας προσαρμογής του widget, απενεργοποιώντας τις ενημερώσεις της αλυσίδας σε πραγματικό χρόνο'; title='Απενεργοποίηση ζωντανών σχολίων' app-screenshot-end]