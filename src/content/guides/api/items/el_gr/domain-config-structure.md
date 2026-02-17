---
Ένα αντικείμενο `DomainConfig` αντιπροσωπεύει τη ρύθμιση για έναν τομέα για έναν tenant.

Η δομή του αντικειμένου `DomainConfig` είναι η εξής:

[inline-code-attrs-start title = 'Δομή του DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Ένας τομέας, όχι URL, όπως "fastcomments.com" ή "www.example.com". Μπορεί να περιλαμβάνεται υποτομέας εάν επιθυμείτε να περιορίσετε σε υποτομέα. Μέγιστο 1000 χαρακτήρες. **/
    domain: string
    /** Το From-Name που χρησιμοποιείται όταν αποστέλλονται emails. **/
    emailFromName?: string
    /** Το From-Email που χρησιμοποιείται όταν αποστέλλονται emails. Βεβαιωθείτε ότι το SPF έχει ρυθμιστεί ώστε να επιτρέπει στο mail.fastcomments.com να αποστέλλει emails ως τον τομέα που χρησιμοποιείται σε αυτό το πεδίο. **/
    emailFromEmail?: string
    /** READONLY. Πότε δημιουργήθηκε το αντικείμενο. **/
    createdAt: string
    /** Το λογότυπο που σχετίζεται με αυτόν τον τομέα. Χρησιμοποιείται σε emails. Χρησιμοποιήστε HTTPS. **/
    logoSrc?: string
    /** Ένα μικρότερο λογότυπο που σχετίζεται με αυτόν τον τομέα. Χρησιμοποιήστε HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. Το URL που χρησιμοποιείται στο υποσέλιδο κάθε αποστελλόμενου email. Υποστηρίζει τη μεταβλητή "[userId]". **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. Τα headers που χρησιμοποιούνται σε κάθε αποστελλόμενο email. Χρήσιμο, για παράδειγμα, για τη ρύθμιση headers σχετικά με το unsubscribe για βελτίωση της παράδοσης. Η καταχώρηση List-Unsubscribe σε αυτό το Record, εάν υπάρχει, υποστηρίζει τη μεταβλητή "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Απενεργοποιήστε όλους τους συνδέσμους διαγραφής εγγραφής. Δεν συνιστάται, μπορεί να επηρεάσει αρνητικά τα ποσοστά παράδοσης. **/
    disableUnsubscribeLinks?: boolean
    /** Ρυθμίσεις DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή ρυθμίσεων DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Το όνομα τομέα στο DKIM record σας. **/
    domainName: string
    /** Ο selector κλειδιού DKIM για χρήση. **/
    keySelector: string
    /** Το δημόσιο κλειδί σε μορφή PEM. Επιστρέφεται σε GET απαντήσεις. **/
    publicKey: string
    /** @deprecated Δεν επιστρέφεται πλέον στις απαντήσεις του API. Γίνεται αποδεκτό κατά την εγγραφή για συμβατότητα με παλαιότερες εκδόσεις. **/
    privateKey?: string
}
[inline-code-end]

### Για έλεγχο ταυτότητας

Η ρύθμιση Domain χρησιμοποιείται για να καθορίσει ποιες τοποθεσίες μπορούν να φιλοξενήσουν το widget FastComments για τον λογαριασμό σας. Πρόκειται για μια βασική μορφή ελέγχου ταυτότητας, που σημαίνει ότι η προσθήκη ή η αφαίρεση οποιασδήποτε ρύθμισης Domain μπορεί να επηρεάσει τη διαθεσιμότητα της εγκατάστασης FastComments σας σε παραγωγή.

Μην αφαιρείτε ή ενημερώνετε την ιδιότητα `domain` ενός `Domain Config` για έναν τομέα που χρησιμοποιείται αυτή τη στιγμή, εκτός αν σκοπεύετε να απενεργοποιήσετε αυτόν τον τομέα.

Αυτό έχει την ίδια συμπεριφορά με την αφαίρεση ενός τομέα από [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Σημειώστε επίσης ότι η αφαίρεση ενός τομέα από το UI `My Domains` θα αφαιρέσει οποιαδήποτε αντίστοιχη ρύθμιση για αυτόν τον τομέα που μπορεί να είχε προστεθεί μέσω αυτού του UI.

### Για προσαρμογή email

Ο σύνδεσμος διαγραφής εγγραφής στο υποσέλιδο του email και η λειτουργία one-click-unsubscribe που προσφέρουν πολλοί email clients μπορούν να διαμορφωθούν μέσω αυτού του API ορίζοντας τα `footerUnsubscribeURL` και `emailHeaders`, αντίστοιχα.

### Για DKIM

Αφού ορίσετε τα DKIM DNS records σας, απλώς ενημερώστε το DomainConfig με τη ρύθμιση DKIM χρησιμοποιώντας τη δομή που ορίζεται. 

---