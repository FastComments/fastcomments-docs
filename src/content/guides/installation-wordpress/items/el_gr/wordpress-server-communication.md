Για να λειτουργήσει το πρόσθετο, ένα token αποθηκεύεται στη βάση δεδομένων του WordPress σας και επίσης στον λογαριασμό σας στο FastComments. Όταν το πρόσθετο κάνει αιτήματα προς τους διακομιστές μας, παρέχει
αυτό το token.

Μπορείτε να δείτε όλες τις ενσωματώσεις που έχουν εξουσιοδοτηθεί για τον λογαριασμό σας στο FastComments [εδώ](https://fastcomments.com/auth/my-account/manage-data/integrations).

Όλη η επικοινωνία γίνεται μέσω HTTPS.

Όλη η επικοινωνία είναι *εξερχόμενη* από τον διακομιστή WordPress σας *προς* το FastComments.com, συμπεριλαμβανομένου του συγχρονισμού *πίσω* στην εγκατάσταση του WordPress σας καθώς υλοποιείται
μέσω [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) από ένα [cron](https://developer.wordpress.org/plugins/cron/) setup στην εγκατάσταση του WordPress σας.