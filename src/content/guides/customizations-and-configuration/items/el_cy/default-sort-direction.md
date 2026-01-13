[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Εξ ορισμού, το FastComments θα ταξινομεί τα σχόλια με την κατεύθυνση ταξινόμησης "Most Relevant".

Η ταξινόμηση "Most Relevant" λαμβάνει υπόψη τον χρόνο που γράφτηκε το σχόλιο και τον αριθμό ψήφων για την ταξινόμηση.

Ο χρήστης μπορεί στη συνέχεια να αλλάξει την κατεύθυνση ταξινόμησης είτε σε "Oldest First" είτε σε "Newest First" στο UI του widget σχολίων.

Ωστόσο, μπορούμε να αλλάξουμε την προεπιλογή σε οποιαδήποτε από τις τρεις. Για παράδειγμα, αν θέλετε να εμφανίζονται πρώτα τα παλαιότερα σχόλια:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Ορίζουμε την τιμή του **defaultSortDirection** σε "OF" για να ορίσουμε την κατεύθυνση σε "OF".

Για την κατεύθυνση ταξινόμησης νεότερα-πρώτα, θα κάναμε το ακόλουθο:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Οι έγκυρες τιμές για το **defaultSortDirection** είναι:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Προεπιλεγμένη Κατεύθυνση Ταξινόμησης".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Σημειώστε ότι τα σχόλια σε κάθε σελίδα για κάθε κατεύθυνση ταξινόμησης υπολογίζονται εκ των προτέρων, επομένως όλες οι κατευθύνσεις ταξινόμησης έχουν την ίδια απόδοση.