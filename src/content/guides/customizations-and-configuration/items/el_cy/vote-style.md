[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Εξ ορισμού, το FastComments θα εμφανίζει επιλογές ψήφου ως βέλη πάνω και κάτω, επιτρέποντας στους χρήστες να ψηφίσουν είτε θετικά είτε αρνητικά ένα σχόλιο.

Ωστόσο, είναι δυνατόν να αλλάξετε το στυλ της γραμμής εργαλείων ψήφου. Οι τρέχουσες επιλογές είναι τα προεπιλεγμένα κουμπιά Up/Down, ή η χρήση ενός μηχανισμού ψήφου με σχήμα Καρδιάς.

Χρησιμοποιούμε τη σημαία **voteStyle** ως εξής:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Συνιστούμε ανεπιφύλακτα να το κάνετε αυτό χωρίς κώδικα καθώς ενεργοποιεί επίσης επικυρώσεις στην πλευρά του διακομιστή. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Μπορείτε επίσης να απενεργοποιήσετε την ψήφιση, δείτε `Disable Voting` παραπάνω στις επιλογές στυλ.