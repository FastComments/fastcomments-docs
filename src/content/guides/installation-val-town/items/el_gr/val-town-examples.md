Τέσσερα δημόσια vals που μπορείτε να κάνετε remix, το καθένα καλύπτει ένα κομμάτι αυτού του οδηγού.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) είναι ένα blog σε Markdown με ένα νήμα σχολίων κάτω από κάθε ανάρτηση και μαζικά μετρητικά σχολίων στην ευρετήρια σελίδα. Λειτουργεί αμέσως μετά το remix, και μία μεταβλητή περιβάλλοντος το δείχνει στον δικό σας λογαριασμό.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) συνδέει τον επισκέπτη με τον λογαριασμό Val Town του και μεταφέρει αυτήν την ταυτότητα στο widget, ώστε να μην χρειάζεται δεύτερη σύνδεση.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) επαληθεύει την υπογραφή HMAC σε κάθε παράδοση και αποθηκεύει τα γεγονότα σε SQLite. Διαθέτει ένα κουμπί που υπογράφει ένα δοκιμαστικό payload και το παραδίδει στον εαυτό του, ώστε να δείτε την επαλήθευση να succeeds πριν ρυθμίσετε ένα πραγματικό webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) είναι μια βιβλιοθήκη δεξιοτήτων agent του FastComments που καλύπτουν το widget, το SSO, το REST API, τον συντονισμό (moderation) και τη μετάβαση από το Disqus. Κάντε το remix και ο agent του Val Town, Townie, τις ανακαλύπτει αυτόματα από το `skills/`, ώστε ο agent σας να ξέρει πώς να συνδέσει τα σχόλια χωρίς να επικολλάτε τεκμηρίωση στη συνομιλία.

Οι ίδιες δεξιότητες εγκαθίστανται οπουδήποτε αλλού με `npx skills add fastcomments/skills`.