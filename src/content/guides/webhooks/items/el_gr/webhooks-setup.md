---
Ακολουθήστε τα ίδια βήματα για το `localhost` όπως θα κάνατε στην παραγωγή. Βεβαιωθείτε ότι έχετε ρυθμίσει τα production domains και τα API Secrets.

Πρώτα, μεταβείτε στο [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Αυτό είναι προσβάσιμο μέσω Manage Data -> Webhooks.

Η σελίδα εμφανίζει κάθε webhook στον λογαριασμό σας:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Σελίδα διαχείρισης Webhooks που εμφανίζει κάθε webhook με το URL, το γεγονός, το domain, τη μέθοδο, την κατάσταση και τον αριθμό των γεγονότων στην ουρά'; title='Λίστα Webhooks'; cacheBuster = 'v4' app-screenshot-end]

Κάντε κλικ στο **New Webhook** για να προσθέσετε ένα. Κάθε webhook έχει ένα URL, ένα γεγονός σχολίου (δημιουργήθηκε, ενημερώθηκε ή διαγράφηκε), ένα domain και μια μέθοδο HTTP:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Φόρμα νέου webhook με πεδία URL, γεγονός, domain και μέθοδο HTTP, καθώς και το κουμπί Send Test Payload'; title='Νέο Webhook'; cacheBuster = 'v4' app-screenshot-end]

Κάθε webhook παραδίδεται ανεξάρτητα. Μπορείτε να στείλετε το ίδιο γεγονός σε πολλαπλά endpoints, και ένα webhook που περιορίζεται σε **All Domains** λαμβάνει σχόλια από κάθε domain ακόμη και όταν υπάρχει ένα domain-specific webhook για το ίδιο γεγονός. Το ίδιο URL, γεγονός και domain δεν μπορούν να προστεθούν δύο φορές.

Πριν αποθηκεύσετε, κάντε κλικ στο **Send Test Payload** για να ελέγξετε αν το endpoint δέχεται μια υπογεγραμμένη αίτηση. Δείτε την επόμενη ενότητα, "Testing", για λεπτομέρειες.

Από τη λίστα μπορείτε να επεξεργαστείτε, να απενεργοποιήσετε, να ενεργοποιήσετε ξανά ή να διαγράψετε ένα webhook. Η απενεργοποίηση διατηρεί τα γεγονότα στην ουρά μέχρι το webhook να ενεργοποιηθεί ξανά· η διαγραφή τα απορρίπτει.

Τα Webhooks μπορούν επίσης να δημιουργηθούν μέσω του API, για παράδειγμα από το Zapier. Αυτά εμφανίζονται στην ίδια λίστα με την πηγή **API**. Δείτε τη διαχείριση Webhooks μέσω του API.

---