[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Ο μέγιστος αριθμός χαρακτήρων που επιτρέπεται να εισαχθεί στο πεδίο εισαγωγής σχολίου μπορεί να περιοριστεί από την παράμετρο **maxCommentCharacterLength**.

Η προεπιλογή είναι 2000.

Τέτοια στοιχεία όπως URLs εικόνων δεν περιλαμβάνονται στον υπολογισμό του μήκους.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Περιορισμός Μήκους Σχολίου'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Πεδίο μεγέθους μέγιστου σχολίου στη σελίδα προσαρμογής widget, που χρησιμοποιείται για τον περιορισμό του αριθμού χαρακτήρων που μπορεί να περιέχει ένα σχόλιο'; title='Περιορισμός Μήκους Σχολίου' app-screenshot-end]

---