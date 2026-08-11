[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα εμφανίζει μια ετικέτα "Μη επαληθευμένο Σχόλιο" για σχόλια που έχουν αφήσει για χρήστη που έχει μια μη επαληθευμένη συνεδρία προγράμματος περιήγησης. Διαβάστε περισσότερα για τα μη επαληθευμένα σχόλια [εδώ](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Απενεργοποίηση της ετικέτας μη επαληθευμένου σχολίου'; code-example-end]

Επιπλέον, αυτή η λειτουργία μπορεί να χρησιμοποιηθεί, χωρίς να γράψετε κώδικα, στη Διεπαφή Προσαρμογής UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Σελίδα προσαρμογής widget με το πλαίσιο ελέγχου Απενεργοποίηση της ετικέτας μη επαληθευμένου σχολίου επιλεγμένο'; title='Απενεργοποίηση της ετικέτας μη επαληθευμένου σχολίου' app-screenshot-end]

---