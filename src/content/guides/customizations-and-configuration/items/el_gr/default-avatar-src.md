[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Όταν ένας χρήστης σχολιάζει με το FastComments για πρώτη φορά, θα προσπαθήσουμε να λάβουμε το avatar του από <a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Ωστόσο, εάν δεν βρούμε avatar, ή ο χρήστης δεν ορίσει ποτέ ένα στο λογαριασμό του, εμφανίζουμε μια στατική προεπιλεγμένη εικόνα avatar.

Για να καθορίσετε τη δική σας στατική εικόνα avatar, μπορείτε να χρησιμοποιήσετε τη ρύθμιση *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Αντικατάσταση του προεπιλεγμένου Avatar'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής widget, δείτε την ενότητα "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Τμήμα προεπιλεγμένου Avatar της σελίδας προσαρμογής widget, όπου ορίζετε τη διεύθυνση URL της εναλλακτικής εικόνας avatar'; title='Προσαρμογή του προεπιλεγμένου Avatar' app-screenshot-end]

Σημειώστε ότι ο ορισμός του avatar για έναν συγκεκριμένο χρήστη, όπως με το SSO, καλύπτεται σε δική του ενότητα.