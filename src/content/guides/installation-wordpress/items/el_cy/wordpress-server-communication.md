Για να λειτουργήσει το plugin, ένα token αποθηκεύεται στη βάση δεδομένων του WordPress σας και επίσης στον λογαριασμό FastComments σας. Όταν το plugin κάνει αίτημα στους διακομιστές μας, παρέχει
αυτό το token.

Μπορείτε να δείτε όλες τις ενσωματώσεις που έχουν εξουσιοδοτηθεί για τον λογαριασμό FastComments σας [εδώ](https://fastcomments.com/auth/my-account/manage-data/integrations).

Όλη η επικοινωνία γίνεται μέσω HTTPS.

Όλη η επικοινωνία είναι *εξερχόμενη* από τον διακομιστή WordPress σας *προς* το FastComments.com, συμπεριλαμβανομένου του συγχρονισμού *πίσω* στην εγκατάσταση WordPress σας καθώς υλοποιείται
μέσω [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) από μια [cron](https://developer.wordpress.org/plugins/cron/) ρύθμιση στην εγκατάσταση WordPress σας.