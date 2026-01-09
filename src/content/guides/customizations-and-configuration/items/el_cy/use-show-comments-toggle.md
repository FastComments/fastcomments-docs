[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα αποδώσει το πλαίσιο εισαγωγής σχολίου και το νήμα σχολίων ταυτόχρονα. Για να εξοικονομήσει κάθετο χώρο,
θα κρύψει επίσης οποιαδήποτε άλλα απαιτούμενα πεδία μέχρι να αλληλεπιδράσει ο χρήστης με το widget.

Ωστόσο, το widget σχολίων μπορεί να κρυφτεί πίσω από ένα κουμπί, για παράδειγμα:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Το κουμπί χρησιμοποιεί διαφορετικό μεταφρασμένο κείμενο ανάλογα με το αν τα σχόλια εμφανίζονται αυτήν τη στιγμή ή όχι. Εάν τα σχόλια είναι κρυμμένα, χρησιμοποιεί το `translations.SHOW_COMMENTS_BUTTON_TEXT`. Εάν τα σχόλια εμφανίζονται, χρησιμοποιεί το `translations.HIDE_COMMENTS_BUTTON_TEXT`. Οι μεταφράσεις μπορούν να περιέχουν το κείμενο `[count]` το οποίο θα αντικατασταθεί με τον τοπικό αριθμό.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Αυτό έχει σχεδιαστεί για να αντικαταστήσει τη ρύθμιση `hideCommentsUnderCountTextFormat`.

Ο αριθμός ενημερώνεται σε πραγματικό χρόνο μαζί με το νήμα σχολίων. Το κουμπί δεν εμφανίζεται εάν δεν υπάρχουν σχόλια.

Αυτό μπορεί να ενεργοποιηθεί χωρίς κώδικα δημιουργώντας έναν κανόνα εξατομίκευσης και ενεργοποιώντας "Κάντε κλικ για εμφάνιση σχολίων":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]


---