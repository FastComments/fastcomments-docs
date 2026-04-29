Ενεργοποιείται όταν ένας moderator εγκρίνει ένα σχόλιο.

### Πλαίσιο που λαμβάνει ο agent

- Το σχόλιο που μόλις εγκρίθηκε.
- The **triggering user ID** - ο moderator που το ενέκρινε.
- Προαιρετικό thread / ιστορικό χρήστη / περιεχόμενο σελίδας όπως έχει ρυθμιστεί.

### Ποιος το ενεργοποιεί

Μια ενέργεια ανθρώπινου moderator.

### Σημειώσεις

- Ένα "approved" σχόλιο είναι ένα **visible** σχόλιο στην ορολογία του FastComments. Δείτε [How Approvals Work](/guide-moderation.html#moderation-approvals) στον οδηγό διαχείρισης για τη διάκριση μεταξύ approved/unapproved και reviewed/unreviewed.
- The trigger fires on approval **transitions**: ένα σχόλιο που πηγαίνει από unapproved σε approved το πυροδοτεί; ένα σχόλιο που ήταν ήδη approved και επανα-αποθηκεύτηκε δεν το κάνει.
- Για tenants όπου τα σχόλια είναι από προεπιλογή auto-approved, αυτό το trigger ενεργοποιείται μόνο όταν ένας moderator επανεγκρίνει ρητά ένα προηγουμένως κρυμμένο σχόλιο.

### Συνηθισμένες χρήσεις

- **Welcome / engagement** - ένας agent μπορεί να απαντήσει σε σχολιαστές που σχολιάζουν για πρώτη φορά τη στιγμή που ένας moderator τους εγκρίνει, αντί κατά τη στιγμή της δημοσίευσης.
- **Cross-agent coordination** - εάν ένας ξεχωριστός agent είχε επισημάνει το σχόλιο για review, η έγκριση είναι το σήμα ότι ο ανθρώπινος έλεγχος ολοκληρώθηκε.
- **Audit trail** μέσω των [Webhooks](#webhooks-overview).