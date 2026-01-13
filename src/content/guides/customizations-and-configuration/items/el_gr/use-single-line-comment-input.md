[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments επιτρέπει στον χρήστη να εισάγει ένα σχόλιο με όσες γραμμές θέλει, έως το προεπιλεγμένο όριο χαρακτήρων.

Ωστόσο, μπορεί να είναι επιθυμητό να περιοριστεί ο χρήστης να εισάγει μόνο μία γραμμή κειμένου. Μερικά παραδείγματα χρήσης περιλαμβάνουν ηλεκτρονικούς πλειστηριασμούς ή ζωντανή συνομιλία, για την οποία το FastComments
μπορεί να χρησιμοποιηθεί.

Ενεργοποιούμε τη σημαία **useSingleLineCommentInput** ως εξής:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Αυτό μπορεί να γίνει και χωρίς κώδικα. Στην σελίδα προσαρμογής του widget, δείτε την ενότητα "Ενεργοποίηση εισαγωγής σχολίου μίας γραμμής".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Σημειώστε ότι τα σχόλια σε κάθε σελίδα για κάθε κατεύθυνση ταξινόμησης υπολογίζονται εκ των προτέρων, οπότε όλες οι κατευθύνσεις ταξινόμησης έχουν την ίδια απόδοση.