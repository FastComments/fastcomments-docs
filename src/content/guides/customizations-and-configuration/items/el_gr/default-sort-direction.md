[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Από προεπιλογή, το FastComments θα ταξινομεί τα σχόλια με την κατεύθυνση ταξινόμησης «Πιο Σχετικό».

Η ταξινόμηση «Πιο Σχετικό» λαμβάνει υπόψη την ώρα που αφήθηκε το σχόλιο και τον αριθμό των ψήφων για την ταξινόμηση.

Ο χρήστης μπορεί στη συνέχεια να αλλάξει την κατεύθυνση ταξινόμησης είτε σε Παλαιότερα είτε σε Νεότερα πρώτα στο περιβάλλον χρήστη (UI) του widget σχολίων.

Ωστόσο, μπορούμε να αλλάξουμε την προεπιλεγμένη τιμή σε οποιαδήποτε από τις τρεις. Για παράδειγμα, αν θέλετε να εμφανίζονται πρώτα τα παλαιότερα σχόλια:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Θέτουμε την τιμή του **defaultSortDirection** σε "OF" για να ορίσουμε την κατεύθυνση σε "OF".

Για την κατεύθυνση ταξινόμησης Νεότερα πρώτα, θα κάναμε το εξής:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Οι έγκυρες τιμές για το **defaultSortDirection** είναι:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα «Προεπιλεγμένη Κατεύθυνση Ταξινόμησης».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Σημειώστε ότι τα σχόλια σε κάθε σελίδα για κάθε κατεύθυνση ταξινόμησης υπολογίζονται εκ των προτέρων, οπότε όλες οι κατευθύνσεις ταξινόμησης έχουν την ίδια απόδοση.