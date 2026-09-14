Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) είναι ένα blog σε Markdown με νήμα κάτω από κάθε ανάρτηση και συνολικές μετρήσεις σχολίων στον δείκτη. Λειτουργεί τη στιγμή που το επαναχρησιμοποιείτε, και μια μεταβλητή περιβάλλοντος το κατευθύνει στον δικό σας λογαριασμό.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) συνδέει τον επισκέπτη με τον λογαριασμό του Val Town και παραδίδει αυτή την ταυτότητα στο widget, ώστε να μην υπάρχει δεύτερη σύνδεση.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) επαληθεύει την υπογραφή HMAC σε κάθε παράδοση και αποθηκεύει τα γεγονότα σε SQLite. Διαθέτει ένα κουμπί που υπογράφει ένα δοκιμαστικό payload και το αποστέλλει στον εαυτό του, ώστε να μπορείτε να δείτε την επαλήθευση να πετυχαίνει πριν ρυθμίσετε ένα πραγματικό webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) είναι μια βιβλιοθήκη δεξιοτήτων πράκτορα FastComments που καλύπτει το widget, το SSO, το REST API, τη διαχείριση και τη μετάβαση από το Disqus. Επαναχρησιμοποιήστε το και ο πράκτορας της Val Town, Townie, αντλεί αυτόματα τις δεξιότητες από το `skills/`, ώστε ο πράκτοράς σας να ξέρει πώς να ενσωματώνει σχόλια χωρίς να επικολλάτε τεκμηρίωση στη συνομιλία.

Οι ίδιες δεξιότητες εγκαθίστανται οπουδήποτε αλλού με `npx skills add fastcomments/skills`.