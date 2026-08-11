[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments επιτρέπει στους χρήστες να αποκλείουν άλλους χρήστες. Η απόκλειση ενός χρήστη θα κάνει τα σχόλιά του να καλύπτονται, αποτρέπει τις ειδοποιήσεις μεταξύ των χρηστών, κ.λπ.

Μπορεί να είναι επιθυμητό να απενεργοποιηθεί αυτή η λειτουργία. Μπορεί να γίνει ως εξής:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Απενεργοποίηση Αποκλεισμού'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα, το οποίο επίσης ενεργοποιεί σωστή επικύρωση από την πλευρά του διακομιστή, μέσω της διεπαφής προσαρμογής Widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Επιλογή απενεργοποίησης αποκλεισμού στη διεπαφή προσαρμογής widget, η οποία εμποδίζει τους χρήστες να αποκλείουν ο ένας τον άλλο'; title='Απενεργοποίηση Αποκλεισμού' app-screenshot-end]