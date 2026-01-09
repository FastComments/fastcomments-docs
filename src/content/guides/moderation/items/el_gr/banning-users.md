Υπάρχουν δύο τρόποι να αποκλείσετε χρήστες από το να σχολιάζουν στον ιστότοπό σας με το FastComments.

Ο πρώτος είναι αν ήδη γνωρίζετε το email τους, μπορείτε να το εισάγετε στη σελίδα <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">αποκλεισμένοι χρήστες</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Αυτή η σελίδα είναι προσβάσιμη μέσω Moderate Comments -> Banned Users

Όταν πρόκειται να αποκλείσουμε έναν χρήστη, μπορούμε να επιλέξουμε τύπο, είτε Permanent είτε Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Ο δεύτερος τρόπος για να αποκλείσετε έναν χρήστη είναι κάνοντας κλικ στο κουμπί αποκλεισμού που βρίσκεται σε κάθε σχόλιο στη σελίδα Comment Moderation.

Όταν κάνουμε κλικ στο κουμπί αποκλεισμού, θα εμφανιστούν κάποιες επιλογές, όπου μπορούμε να καθορίσουμε τον τύπο αποκλεισμού και τη διάρκεια.

### Σκιώδεις Αποκλεισμοί

Ένας σκιώδης αποκλεισμός είναι ένας τύπος αποκλεισμού που κάνει να φαίνεται ότι το σχόλιο ή η ψήφος του χρήστη αποθηκεύτηκε επιτυχώς, ενώ στην πραγματικότητα δεν έγινε. Αυτό μπορεί να είναι επιθυμητό σε ορισμένες περιπτώσεις.

### Αποκλεισμός μέσω Διεύθυνσης IP

Εκτός εάν ένας tenant επιθυμεί να εξαιρεθεί (opt out), το FastComments υποστηρίζει τον αποκλεισμό μέσω IP αποθηκεύοντας μια κατακερματισμένη (hashed) έκδοση της διεύθυνσης IP του σχολιαστή.