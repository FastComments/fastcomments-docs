Ένα αντικείμενο `DomainConfig` αντιπροσωπεύει τη διαμόρφωση για ένα domain για έναν tenant.

Η δομή για το αντικείμενο `DomainConfig` έχει ως εξής:

[inline-code-attrs-start title = 'Δομή DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Ένας domain, όχι URL, όπως "fastcomments.com" ή "www.example.com". Μπορεί να συμπεριληφθεί υποτομέας αν επιθυμείτε περιορισμό σε υποτομέα. Μέγιστο 1000 χαρακτήρες. **/
    domain: string
    /** Το From-Name που χρησιμοποιείται όταν αποστέλλονται emails. **/
    emailFromName?: string
    /** Το From-Email που χρησιμοποιείται όταν αποστέλλονται email. Βεβαιωθείτε ότι το SPF είναι ρυθμισμένο ώστε να επιτρέπει στο mail.fastcomments.com να στέλνει email ως το domain που χρησιμοποιείται σε αυτήν την ιδιότητα. **/
    emailFromEmail?: string
    /** READONLY. Πότε δημιουργήθηκε το αντικείμενο. **/
    createdAt: string
    /** Το λογότυπο που σχετίζεται με αυτό το domain. Χρησιμοποιείται σε emails. Χρησιμοποιήστε HTTPS. **/
    logoSrc?: string
    /** Ένα μικρότερο λογότυπο σχετικό με αυτό το domain. Χρησιμοποιήστε HTTPS. **/
    logoSrc100px?: string
    /** ΜΟΝΟ SSO. Το URL που χρησιμοποιείται στο υποσέλιδο κάθε αποστελλόμενου email. Υποστηρίζει τη μεταβλητή "[userId]". **/
    footerUnsubscribeURL?: string
    /** ΜΟΝΟ SSO. Οι κεφαλίδες που χρησιμοποιούνται σε κάθε αποστελλόμενο email. Χρήσιμο, για παράδειγμα, για τον ορισμό κεφαλίδων σχετικών με τη διαγραφή συνδρομής για βελτίωση της παράδοσης. Η εγγραφή List-Unsubscribe σε αυτό το Record, εάν υπάρχει, υποστηρίζει τη μεταβλητή "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Απενεργοποιεί όλους τους συνδέσμους διαγραφής συνδρομής. Δεν συνιστάται, μπορεί να επηρεάσει αρνητικά τα ποσοστά παράδοσης. **/
    disableUnsubscribeLinks?: boolean
    /** Διαμόρφωση DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή ρυθμίσεων DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Το όνομα domain στην καταγραφή DKIM σας. **/
    domainName: string
    /** Ο selector κλειδιού DKIM που θα χρησιμοποιηθεί. **/
    keySelector: string
    /** Το δημόσιο κλειδί, σε μορφή PEM. Επιστρέφεται στις GET απαντήσεις. **/
    publicKey: string
    /** @deprecated Δεν επιστρέφεται πλέον στις απαντήσεις του API. Γίνεται αποδεκτό κατά την εγγραφή για συμβατότητα προς τα πίσω. **/
    privateKey?: string
}
[inline-code-end]

### Για Έλεγχο Ταυτότητας

Η διαμόρφωση domain χρησιμοποιείται για να καθορίσει ποιες τοποθεσίες μπορούν να φιλοξενήσουν το widget του FastComments για τον λογαριασμό σας. Πρόκειται για μια βασική μορφή ελέγχου ταυτότητας, πράγμα που σημαίνει ότι η προσθήκη ή η αφαίρεση οποιασδήποτε διαμόρφωσης domain μπορεί να επηρεάσει τη διαθεσιμότητα της εγκατάστασης FastComments στην παραγωγή.

Μην αφαιρείτε ή ενημερώνετε την ιδιότητα `domain` ενός `Domain Config` για ένα domain που χρησιμοποιείται επί του παρόντος, εκτός αν σκοπεύετε να απενεργοποιήσετε αυτό το domain.

Αυτό έχει την ίδια συμπεριφορά με την αφαίρεση ενός domain από [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Σημειώστε επίσης ότι η αφαίρεση ενός domain από το UI `My Domains` θα αφαιρέσει οποιαδήποτε αντίστοιχη διαμόρφωση για εκείνο το domain που ενδέχεται να είχε προστεθεί μέσω αυτού του UI.

### Για Προσαρμογή Email

Ο σύνδεσμος διαγραφής συνδρομής στο υποσέλιδο του email, καθώς και η λειτουργία απεγγραφής με ένα κλικ που προσφέρεται από πολλούς πελάτες email, μπορούν να ρυθμιστούν μέσω αυτού του API ορίζοντας αντίστοιχα τα `footerUnsubscribeURL` και `emailHeaders`.

### Για DKIM

Αφού ορίσετε τις καταχωρήσεις DKIM DNS σας, απλά ενημερώστε το DomainConfig με τη διαμόρφωση DKIM χρησιμοποιώντας τη δομή που ορίζεται παραπάνω.

---