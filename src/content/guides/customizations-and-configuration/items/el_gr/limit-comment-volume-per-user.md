Κατά προεπιλογή, κάθε χρήστης μπορεί να υποβάλει έως και `5 comments` μέσα στο ίδιο λεπτό.

Αυτό παρακολουθείται από user id, anon user id, και ip address (κατακερματισμένο).

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Σημειώστε ότι αν χρησιμοποιείτε το comment creation API, ίσως θελήσετε να περάσετε τη αρχική διεύθυνση `ip` του χρήστη στο αίτημα προς το backend μας ώστε ο περιορισμός ρυθμού να εφαρμόζεται ανά χρήστη και όχι παγκοσμίως για τον λογαριασμό σας.