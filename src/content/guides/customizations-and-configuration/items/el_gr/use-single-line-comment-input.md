[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα επιτρέψει στον χρήστη να εισάγει ένα σχόλιο με όσες γραμμές θέλει, μέχρι το προεπιλεγμένο όριο χαρακτήρων.

Ωστόσο, μπορεί να είναι επιθυμητό να περιοριστεί ο χρήστης στην εισαγωγή μόνο μιας γραμμής κειμένου. Μερικά παραδείγματα χρήσης περιλαμβάνουν διαδικτυακές δημοπρασίες ή ζωντανή συνομιλία, για τα οποία μπορεί να χρησιμοποιηθεί το FastComments.

Ενεργοποιούμε τη σημαία **useSingleLineCommentInput** ως εξής:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Ενεργοποίηση Εισαγωγής Σχολίου Μίας Γραμμής'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής widget, δείτε την ενότητα "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Το κουτάκι ελέγχου εισαγωγής σχολίου μίας γραμμής ενεργοποιήθηκε στη σελίδα προσαρμογής widget, περιορίζοντας την εισαγωγή σε μία γραμμή'; title='Ενεργοποίηση Εισαγωγής Σχολίου Μίας Γραμμής' app-screenshot-end]

Σημειώστε ότι τα σχόλια σε κάθε σελίδα για κάθε κατεύθυνση ταξινόμησης προ-υπολογίζονται, έτσι όλες οι κατευθύνσεις ταξινόμησης έχουν την ίδια απόδοση.