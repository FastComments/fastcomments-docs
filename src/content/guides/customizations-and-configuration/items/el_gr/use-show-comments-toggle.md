[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα αποδίδει το πλαίσιο εισαγωγής σχολίου και τη νήμα σχολίων ταυτόχρονα. Για να εξοικονομήσει κάποιο κάθετο χώρο,
θα κρύβει επίσης τυχόν άλλα απαιτούμενα πεδία μέχρι να αλληλεπιδράσει ο χρήστης με το widget.

Ωστόσο, το widget σχολίων μπορεί να κρύβεται πίσω από ένα κουμπί, για παράδειγμα:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Widget σχολίων συμπτυγμένο πίσω από ένα κουμπί που εμφανίζει τον αριθμό σχολίων μέχρι ο αναγνώστης να κάνει κλικ'; title='Κλικ για Εμφάνιση Σχολίων' app-screenshot-end]

Το κουμπί χρησιμοποιεί διαφορετικό μεταφρασμένο κείμενο ανάλογα με το αν τα σχόλια εμφανίζονται αυτή τη στιγμή ή όχι. Αν τα σχόλια είναι κρυμμένα, χρησιμοποιεί `translations.SHOW_COMMENTS_BUTTON_TEXT`. Αν το
σχόλια είναι εμφανισμένα, χρησιμοποιεί `translations.HIDE_COMMENTS_BUTTON_TEXT`. Οι μεταφράσεις μπορούν να περιέχουν το κείμενο `[count]` που θα
αντικατασταθεί με τον τοπικοποιημένο αριθμό.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Κλικ για Εμφάνιση ή Απόκρυψη Σχολίων'; code-example-end]

Αυτό έχει σχεδιαστεί για να αντικαταστήσει τη ρύθμιση `hideCommentsUnderCountTextFormat`.

Ο αριθμός ενημερώνεται ζωντανά με τη νήμα σχολίων. Το κουμπί δεν εμφανίζεται εάν δεν υπάρχουν σχόλια.

Αυτό μπορεί να ενεργοποιηθεί χωρίς κώδικα δημιουργώντας έναν κανόνα προσαρμογής και ενεργοποιώντας το "Κλικ για Εμφάνιση Σχολίων":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Το πλαίσιο ελέγχου «Κλικ για Εμφάνιση Σχολίων» επιλεγμένο σε έναν κανόνα προσαρμογής στη σελίδα προσαρμογής widget'; title='Ενεργοποίηση Κλικ για Εμφάνιση Σχολίων' app-screenshot-end]