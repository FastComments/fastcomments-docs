Κατά προεπιλογή, το FastComments θα εμφανίζει το όνομα του χρήστη όπως το εισήγαγε, ή όπως μας παραδόθηκε μέσω SSO.

Ωστόσο, μπορεί να θέλετε να αποκρύψετε ή να εμφανίσετε το όνομα του χρήστη με διαφορετικό τρόπο. Για παράδειγμα, αν το όνομα του χρήστη είναι Allen Rex, ίσως θέλετε να εμφανίζεται μόνο "Allen R.".

Αυτό μπορεί να γίνει χωρίς κώδικα στο UI Προσαρμογής Widget, υπό την ρύθμιση με όνομα `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Οι διαθέσιμες μορφές είναι:

- Capitalize (εμφανίζει τον Example User ως Example User)
- Last Initial (εμφανίζει τον Example User ως Example U.)
- All Initials (εμφανίζει τον Example User ως E. U.)
- Show "Anonymous"

Η αλλαγή εφαρμόζεται άμεσα. Οι χρήστες θα βλέπουν ακόμη το πλήρες όνομα χρήστη στην κορυφή της περιοχής σχολίων για τον εαυτό τους, αλλά τα σχόλιά τους θα εμφανίζουν το τροποποιημένο όνομα χρήστη.

Τα ονόματα χρηστών αποκρύπτονται στην πλευρά του διακομιστή για να προστατευτούν οι χρήστες.