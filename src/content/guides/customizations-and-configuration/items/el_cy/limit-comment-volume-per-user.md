---
Κατά προεπιλογή, κάθε χρήστης μπορεί να υποβάλει έως `5 comments` στο ίδιο λεπτό.

Αυτό παρακολουθείται με βάση το user id, το anon user id και το ip address (hashed).

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Σημειώστε ότι αν χρησιμοποιείτε το comment creation API ίσως θελήσετε να περάσετε το αρχικό `ip` address του χρήστη στο αίτημα προς το backend μας, ώστε ο περιορισμός ρυθμού να εφαρμόζεται ανά χρήστη και όχι συνολικά στον λογαριασμό σας.

---