[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα εμφανίζει τα διακριτικά των χρηστών μόνο στα σχόλιά τους μέσα στη ροή σχολίων.

Ωστόσο, μπορούμε να εμφανίσουμε τα διακριτικά των χρηστών δίπλα στο όνομά τους πάνω από τη φόρμα σχολίου ενεργοποιώντας αυτή τη δυνατότητα στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Αυτό θα εμφανίσει τα διακριτικά του χρήστη δίπλα στο όνομά του στην επάνω γραμμή, κάνοντας τα επιτεύγματά του και την κατάστασή του πιο εμφανή όταν συντάσσει ένα σχόλιο.

Σημειώστε ότι αυτή η δυνατότητα πρέπει να είναι ενεργοποιημένη στο UI προσαρμογής του widget για να λειτουργήσει. Μπορείτε προαιρετικά να ορίσετε τη σημαία **showBadgesInTopBar** σε false στη ρύθμιση της διαμόρφωσης του κώδικά σας για να την απενεργοποιήσετε επιλεκτικά ακόμη και όταν είναι ενεργοποιημένη στο επίπεδο του server:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]