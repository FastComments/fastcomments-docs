[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Κατά προεπιλογή, το FastComments επιτρέπει στους χρήστες να αποκλείουν άλλους χρήστες. Ο αποκλεισμός ενός χρήστη θα έχει ως αποτέλεσμα τα σχόλιά του να αποκρύπτονται, θα αποτρέπει τις ειδοποιήσεις μεταξύ των χρηστών, κ.λπ.

Ενδέχεται να είναι επιθυμητό να απενεργοποιηθεί αυτή η λειτουργία. Αυτό μπορεί να γίνει ως εξής:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα, κάτι που ενεργοποιεί και την κατάλληλη επικύρωση στην πλευρά του διακομιστή, μέσω της διεπαφής προσαρμογής του Widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---