Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, περιγραφόμενο σε TypeScript παρακάτω.

#### Δομή του Αντικειμένου WebhookComment

##### Η Δομή του συμβάντος "Create"
Το σώμα του αιτήματος για το συμβάν "create" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του συμβάντος "Update"
Το σώμα του αιτήματος για το συμβάν "update" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του συμβάντος "Delete"
Το σώμα του αιτήματος για το συμβάν "delete" είναι ένα αντικείμενο WebhookComment.

    Αλλαγή από 14 Νοεμβρίου 2023
    Πριν, το σώμα του αιτήματος για το συμβάν "delete" περιείχε μόνο το id του σχολίου. Τώρα περιέχει ολόκληρο το σχόλιο κατά τη στιγμή της διαγραφής.


[inline-code-attrs-start title = 'Το αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Το id του σχολίου. **/
    id: string
    /** Το id ή το URL που προσδιορίζει το thread των σχολίων. Κανονικοποιημένο. **/
    urlId: string
    /** Το URL που δείχνει το σημείο όπου καταχώρηθηκε το σχόλιο. **/
    url?: string
    /** Το id του χρήστη που άφησε το σχόλιο. Αν είναι SSO, έχει προθεματισμένο το id του tenant. **/
    userId?: string
    /** Το email του χρήστη που άφησε το σχόλιο. **/
    commenterEmail?: string
    /** Το όνομα του χρήστη που εμφανίζεται στο widget σχολίων. Με SSO, μπορεί να είναι displayName. **/
    commenterName: string
    /** Ακατέργαστο κείμενο σχολίου. **/
    comment: string
    /** Κείμενο σχολίου μετά την ανάλυση/επεξεργασία. **/
    commentHTML: string
    /** Εξωτερικό id του σχολίου. **/
    externalId?: string
    /** Το id του γονικού σχολίου. **/
    parentId?: string | null
    /** Η ημερομηνία σε UTC όταν καταχωρήθηκε το σχόλιο. **/
    date: UTC_ISO_DateString
    /** Συνδυασμένο karma (up - down) των ψήφων. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True αν ο χρήστης ήταν συνδεδεμένος όταν σχολίασε, ή επαλήθευσε το σχόλιο, ή αν επαλήθευσε τη συνεδρία του όταν καταχωρήθηκε το σχόλιο. **/
    verified: boolean
    /** Ημερομηνία κατά την οποία επαληθεύτηκε το σχόλιο. **/
    verifiedDate?: number
    /** Αν ένας moderator σήμανε το σχόλιο ως αναθεωρημένο. **/
    reviewed: boolean
    /** Η τοποθεσία, ή η base64 κωδικοποίηση, του avatar. Θα είναι base64 μόνο αν αυτή ήταν η τιμή που δόθηκε με SSO. **/
    avatarSrc?: string
    /** Το σχόλιο χαρακτηρίστηκε χειροκίνητα ή αυτόματα ως spam; **/
    isSpam: boolean
    /** Το σχόλιο χαρακτηρίστηκε αυτόματα ως spam; **/
    aiDeterminedSpam: boolean
    /** Υπάρχουν εικόνες στο σχόλιο; **/
    hasImages: boolean
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για την ταξινόμηση "Most Relevant". **/
    pageNumber: number
    /** Ο αριθμός σελίδας για την ταξινόμηση "Oldest First". **/
    pageNumberOF: number
    /** Ο αριθμός σελίδας για την ταξινόμηση "Newest First". **/
    pageNumberNF: number
    /** Το σχόλιο εγκρίθηκε αυτόματα ή χειροκίνητα; **/
    approved: boolean
    /** Ο κωδικός τοπικής ρύθμισης (μορφή: en_us) του χρήστη όταν γράφτηκε το σχόλιο. **/
    locale: string
    /** Οι @mentions που γράφτηκαν στο σχόλιο και αναλύθηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Το domain από το οποίο προέρχεται το σχόλιο. **/
    domain?: string
    /** Η προαιρετική λίστα με τα ids των ομάδων moderation που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Όταν οι χρήστες ετικετοποιούνται σε ένα σχόλιο, οι πληροφορίες αποθηκεύονται σε μια λίστα που ονομάζεται `mentions`. Κάθε αντικείμενο σε αυτή τη λίστα έχει την ακόλουθη δομή.

[inline-code-attrs-start title = 'Το αντικείμενο Mentions του Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το id του χρήστη. Για χρήστες SSO, θα έχει προθεματισμένο το tenant id σας. **/
    id: string
    /** Το τελικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπος χρήστη ετικετοποιήθηκε. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Αν ο χρήστης επιλέξει να μην λαμβάνει ειδοποιήσεις, αυτό θα παραμείνει ρυθμισμένο σε true. **/
    sent: boolean
}
[inline-code-end]

#### Μέθοδοι HTTP

Μπορείτε να ρυθμίσετε τη μέθοδο HTTP για κάθε τύπο γεγονότος webhook στο πάνελ διαχείρισης:

- **Create Event**: POST ή PUT (προεπιλογή: PUT)
- **Update Event**: POST ή PUT (προεπιλογή: PUT)
- **Delete Event**: DELETE, POST ή PUT (προεπιλογή: DELETE)

Εφόσον όλα τα αιτήματα περιέχουν ένα ID, οι λειτουργίες Create και Update είναι από προεπιλογή ιδιοδυναμικές (idempotent) (PUT). Η επανάληψη του ίδιου αιτήματος Create ή Update δεν θα πρέπει να δημιουργήσει διπλότυπα αντικείμενα στην πλευρά σας.

#### Επικεφαλίδες Αιτήματος

Κάθε αίτημα webhook περιλαμβάνει τις ακόλουθες επικεφαλίδες:

| Επικεφαλίδα | Περιγραφή |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (δευτερόλεπτα) όταν υπογράφτηκε το αίτημα |
| `X-FastComments-Signature` | HMAC-SHA256 υπογραφή (`sha256=<hex>`) |

Δείτε [Ασφάλεια & API Tokens](/guide-webhooks.html#webhooks-api-tokens) για πληροφορίες σχετικά με την επαλήθευση της υπογραφής HMAC.