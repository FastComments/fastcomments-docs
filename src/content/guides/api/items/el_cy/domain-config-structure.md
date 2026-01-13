Ένα αντικείμενο `DomainConfig` αντιπροσωπεύει τη διαμόρφωση για ένα domain για έναν ενοικιαστή.

Η δομή για το αντικείμενο `DomainConfig` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Domain Config'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string
    /** The From-Name used when sending emails. **/
    emailFromName?: string
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string
    /** A smaller logo related to this domain. Use HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Ρύθμισης DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** Your private key. Start with -----BEGIN PRIVATE KEY----- and end with -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Για Πιστοποίηση

Η Διαμόρφωση Domain χρησιμοποιείται για να καθοριστεί ποιοι ιστότοποι μπορούν να φιλοξενήσουν το widget FastComments για τον λογαριασμό σας. Αυτή είναι μια βασική μορφή
πιστοποίησης, που σημαίνει ότι η προσθήκη ή αφαίρεση οποιωνδήποτε Διαμορφώσεων Domain μπορεί να επηρεάσει τη διαθεσιμότητα της εγκατάστασης FastComments σας
στην παραγωγή.

Μην αφαιρείτε ή ενημερώνετε την ιδιότητα `domain` ενός `Domain Config` για ένα domain που χρησιμοποιείται αυτή τη στιγμή εκτός αν η απενεργοποίηση αυτού του domain είναι επιθυμητή.

Αυτό έχει την ίδια συμπεριφορά με την αφαίρεση ενός domain από το [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Επίσης σημειώστε ότι η αφαίρεση ενός domain από τη διεπαφή `My Domains` θα αφαιρέσει οποιαδήποτε αντίστοιχη διαμόρφωση για αυτό το domain που μπορεί να έχει προστεθεί μέσω αυτής της διεπαφής.

### Για Προσαρμογή Email

Ο σύνδεσμος διαγραφής εγγραφής στο υποσέλιδο του email, και η λειτουργία διαγραφής εγγραφής με ένα κλικ που προσφέρεται από πολλούς πελάτες email, μπορούν να διαμορφωθούν μέσω αυτού του API ορίζοντας τα `footerUnsubscribeURL` και `emailHeaders`, αντίστοιχα.

### Για DKIM

Αφού ορίσετε τις εγγραφές DNS DKIM σας, απλά ενημερώστε το DomainConfig με τη διαμόρφωση DKIM σας χρησιμοποιώντας την καθορισμένη δομή.
