---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, οι λειτουργίες μορφοποίησης στο FastComments εκτελούνται προσθέτοντας ορατές ετικέτες anchor όπως `<b></b>` γύρω από το κείμενό σας. Το πάτημα στη γραμμή εργαλείων
ή η χρήση συντομεύσεων το κάνει αυτό για εσάς. Ωστόσο, κάποιες κοινότητες ενδέχεται να θέλουν να επιλέξουν τη χρήση μορφοποίησης χωρίς ετικέτες anchor. Αυτό ονομάζεται ενεργοποίηση του
WYSIWYG (αυτό που βλέπεις είναι αυτό που παίρνεις) επεξεργαστή. Αυτός ο επεξεργαστής φαίνεται ακριβώς όπως ο προεπιλεγμένος, εκτός του ότι φορτώνει κάποιο
επιπλέον κώδικα που επιτρέπει στους χρήστες να κάνουν έντονη γραφή, υπογράμμιση κ.λπ. στο κείμενό τους χωρίς ορατές ετικέτες anchor.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---