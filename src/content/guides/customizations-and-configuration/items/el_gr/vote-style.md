[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Από προεπιλογή, το FastComments θα εμφανίζει τις επιλογές ψήφου ως βέλη προς τα πάνω και προς τα κάτω, επιτρέποντας στους χρήστες είτε να ψηφίζουν θετικά είτε αρνητικά ένα σχόλιο.

Ωστόσο, είναι δυνατό να αλλάξετε το στυλ της γραμμής εργαλείων ψήφου. Οι τρέχουσες επιλογές είναι τα προεπιλεγμένα κουμπιά Up/Down, ή η χρήση ενός μηχανισμού ψήφου στυλ Heart.

Χρησιμοποιούμε τη σημαία **voteStyle** ως εξής:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Συνιστούμε θερμά να το κάνετε αυτό χωρίς κώδικα, καθώς ενεργοποιεί επίσης επικυρώσεις στην πλευρά του διακομιστή. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Οι ψήφοι μπορούν επίσης να απενεργοποιηθούν, δείτε το `Disable Voting` παραπάνω από τις επιλογές στυλ.