Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, που περιγράφεται σε TypeScript παρακάτω.

#### Δομή του αντικειμένου WebhookComment

##### Δομή του συμβάντος "create"
Το σώμα του αιτήματος για το συμβάν "create" είναι ένα αντικείμενο WebhookComment.

##### Δομή του συμβάντος "update"
Το σώμα του αιτήματος για το συμβάν "update" είναι ένα αντικείμενο WebhookComment.

##### Δομή του συμβάντος "delete"
Το σώμα του αιτήματος για το συμβάν "delete" είναι ένα αντικείμενο WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Το αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Το id του σχολίου. **/
    id: string
    /** Το id ή το URL που αναγνωρίζει το νήμα σχολίων. Κανονικοποιημένο. **/
    urlId: string
    /** Το URL που δείχνει το μέρος όπου καταχωρήθηκε το σχόλιο. **/
    url?: string
    /** Το id χρήστη που άφησε το σχόλιο. Σε περίπτωση SSO, έχει προθεματισμένο το id του tenant. **/
    userId?: string
    /** Το email του χρήστη που άφησε το σχόλιο. **/
    commenterEmail?: string
    /** Το όνομα του χρήστη που εμφανίζεται στο widget σχολίων. Με SSO, μπορεί να είναι displayName. **/
    commenterName: string
    /** Ακατέργαστο κείμενο σχολίου. **/
    comment: string
    /** Το κείμενο του σχολίου μετά την ανάλυση. **/
    commentHTML: string
    /** Εξωτερικό id σχολίου. **/
    externalId?: string
    /** Το id του γονικού σχολίου. **/
    parentId?: string | null
    /** Η ημερομηνία UTC όταν καταχωρήθηκε το σχόλιο. **/
    date: UTC_ISO_DateString
    /** Συνδυασμένο karma (up - down) των ψήφων. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True εάν ο χρήστης ήταν συνδεδεμένος όταν σχολίασε, ή είχε επαληθεύσει το σχόλιο, ή είχε επαληθεύσει τη συνεδρία του όταν καταχωρήθηκε το σχόλιο. **/
    verified: boolean
    /** Ημερομηνία κατά την οποία επαληθεύτηκε το σχόλιο. **/
    verifiedDate?: number
    /** Εάν ένας συντονιστής σήμανε το σχόλιο ως ελεγμένο. **/
    reviewed: boolean
    /** Η τοποθεσία, ή η κωδικοποίηση base64, του avatar. Θα είναι base64 μόνο αν αυτή ήταν η τιμή που δόθηκε με SSO. **/
    avatarSrc?: string
    /** Το σχόλιο επισημάνθηκε ως ανεπιθύμητο χειροκίνητα ή αυτόματα; **/
    isSpam: boolean
    /** Το σχόλιο επισημάνθηκε αυτόματα ως ανεπιθύμητο; **/
    aiDeterminedSpam: boolean
    /** Υπάρχουν εικόνες στο σχόλιο; **/
    hasImages: boolean
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για την ταξινόμηση "Most Relevant". **/
    pageNumber: number
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για την ταξινόμηση "Oldest First". **/
    pageNumberOF: number
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για την ταξινόμηση "Newest First". **/
    pageNumberNF: number
    /** Το σχόλιο εγκρίθηκε αυτόματα ή χειροκίνητα; **/
    approved: boolean
    /** Ο κωδικός τοπικής ρύθμισης (μορφή: en_us) του χρήστη όταν γράφτηκε το σχόλιο. **/
    locale: string
    /** Τα @mentions που γράφτηκαν στο σχόλιο και αναλύθηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Το domain από το οποίο προέρχεται το σχόλιο. **/
    domain?: string
    /** Η προαιρετική λίστα με τα ids ομάδων moderation που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Όταν χρήστες επισημαίνονται σε ένα σχόλιο, οι πληροφορίες αποθηκεύονται σε μια λίστα που ονομάζεται `mentions`. Κάθε αντικείμενο σε αυτή τη λίστα
έχει την ακόλουθη δομή.

[inline-code-attrs-start title = 'Το αντικείμενο Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το id του χρήστη. Για χρήστες SSO, αυτό θα έχει προθεματισμένο το id του tenant σας. **/
    id: string
    /** Το τελικό κείμενο της ετικέτας @mention, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό κείμενο της ετικέτας @mention, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπου χρήστης επισημάνθηκε. user = λογαριασμός FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Εάν ο χρήστης επιλέξει να μην λαμβάνει ειδοποιήσεις, αυτό θα παραμένει true. **/
    sent: boolean
}
[inline-code-end]

#### Μέθοδοι HTTP

Μπορείτε να ρυθμίσετε τη μέθοδο HTTP για κάθε τύπο συμβάντος webhook στον πίνακα διαχείρισης:

- **Συμβάν Create**: POST ή PUT (προεπιλογή: PUT)
- **Συμβάν Update**: POST ή PUT (προεπιλογή: PUT)
- **Συμβάν Delete**: DELETE, POST, ή PUT (προεπιλογή: DELETE)

Εφόσον όλα τα αιτήματα περιέχουν ένα ID, οι λειτουργίες Create και Update είναι idempotent από προεπιλογή (PUT). Η επανάληψη του ίδιου αιτήματος Create ή Update δεν θα πρέπει να δημιουργεί διπλότυπα αντικείμενα από την πλευρά σας.

#### Επικεφαλίδες Αιτήματος

Κάθε αίτημα webhook περιλαμβάνει τις ακόλουθες επικεφαλίδες:

| Header | Περιγραφή |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Το API Secret σας |
| `X-FastComments-Timestamp` | Unix timestamp (δευτερόλεπτα) όταν υπογράφηκε το αίτημα |
| `X-FastComments-Signature` | Υπογραφή HMAC-SHA256 (`sha256=<hex>`) |

Δείτε [Security & API Tokens](/guides/webhooks/webhooks-api-tokens) για πληροφορίες σχετικά με την επαλήθευση της υπογραφής HMAC.

---