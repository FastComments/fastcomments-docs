[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments επιτρέπει στους χρήστες να αποκλείουν άλλους χρήστες. Ο αποκλεισμός ενός χρήστη θα κάνει τα σχόλιά του να αποκρύπτονται, θα αποτρέπει τις ειδοποιήσεις μεταξύ των χρηστών και ούτω καθεξής.

Μπορεί να είναι επιθυμητό να απενεργοποιηθεί αυτή η λειτουργία. Αυτό μπορεί να γίνει ως εξής:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα, κάτι που επίσης ενεργοποιεί τη σωστή επαλήθευση στην πλευρά του διακομιστή, μέσω της διεπαφής προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]