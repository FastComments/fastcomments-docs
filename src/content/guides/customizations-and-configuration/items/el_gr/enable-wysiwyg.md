[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Σε προεπιλογή, οι λειτουργίες μορφοποίησης στο FastComments πραγματοποιούνται προσθέτοντας ορατά tags αγκύρωσης όπως `<b></b>` γύρω από το κείμενό σας. Το κλικ στη γραμμή εργαλείων
ή η χρήση συντομεύσεων το κάνει αυτό για εσάς. Ωστόσο, κάποιες κοινότητες μπορεί να θέλουν να επιλέξουν τη χρήση μορφοποίησης χωρίς tags αγκύρωσης. Αυτό ονομάζεται ενεργοποίηση του
WYSIWYG (ό,τι βλέπετε είναι ό,τι παίρνετε) επεξεργαστή. Αυτός ο επεξεργαστής φαίνεται ακριβώς ο ίδιος με τον προεπιλεγμένο, εκτός από το ότι φορτώνει κάποιο
επιπλέον κώδικα που επιτρέπει στους χρήστες να κάνουν έντονη, υπογράμμιση κ.λπ. στο κείμενό τους χωρίς ορατά tags αγκύρωσης.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Ενεργοποίηση Προηγμένης Μορφοποίησης".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---