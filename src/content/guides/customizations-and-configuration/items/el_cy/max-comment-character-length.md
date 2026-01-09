[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Ο μέγιστος αριθμός χαρακτήρων που επιτρέπεται να εισαχθούν στο πεδίο εισαγωγής σχολίου μπορεί να περιοριστεί από την παράμετρο **maxCommentCharacterLength**.

Η προεπιλεγμένη τιμή είναι 2000.

Στοιχεία όπως οι διευθύνσεις URL εικόνων δεν περιλαμβάνονται στον υπολογισμό του μήκους.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]