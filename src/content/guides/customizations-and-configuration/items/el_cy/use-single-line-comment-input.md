[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα επιτρέψει στον χρήστη να εισάγει ένα σχόλιο με όσες γραμμές θέλει, μέχρι το προεπιλεγμένο όριο χαρακτήρων.

Ωστόσο, μπορεί να είναι επιθυμητό να περιορίσετε τον χρήστη ώστε να εισάγει μόνο μία γραμμή κειμένου. Μερικά παραδείγματα χρήσης περιλαμβάνουν ηλεκτρονικές δημοπρασίες ή ζωντανή συνομιλία, για τις οποίες το FastComments
μπορεί να χρησιμοποιηθεί.

Ενεργοποιούμε τη **useSingleLineCommentInput** σημαία ως εξής:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Ενεργοποίηση εισαγωγής σχολίου μίας γραμμής".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Σημειώστε ότι, τα σχόλια σε κάθε σελίδα για κάθε κατεύθυνση ταξινόμησης υπολογίζονται προ-υπολογισμένα, οπότε όλες οι κατευθύνσεις ταξινόμησης έχουν την ίδια απόδοση.