Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, περιγραφόμενο σε TypeScript παρακάτω.

#### Δομή του αντικειμένου WebhookComment

##### Δομή του συμβάντος "Create"
Το σώμα του αιτήματος για το συμβάν "create" είναι ένα αντικείμενο WebhookComment.

##### Δομή του συμβάντος "Update"
Το σώμα του αιτήματος για το συμβάν "update" είναι ένα αντικείμενο WebhookComment.

##### Δομή του συμβάντος "Delete"
Το σώμα του αιτήματος για το συμβάν "delete" είναι ένα αντικείμενο WebhookComment.

    Αλλαγή από 14 Νοεμβρίου 2023
    Προηγουμένως το σώμα του αιτήματος για το συμβάν "delete" περιείχε μόνο το id του σχολίου. Τώρα περιέχει ολόκληρο το σχόλιο κατά τη στιγμή της διαγραφής.


[inline-code-attrs-start title = 'Το αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Το id του σχολίου. **/
    id: string
    /** Το id ή το URL που ταυτοποιεί το νήμα σχολίων. Κανονικοποιημένο. **/
    urlId: string
    /** Το URL που δείχνει στο σημείο όπου αφήθηκε το σχόλιο. **/
    url?: string
    /** Το id του χρήστη που άφησε το σχόλιο. Αν είναι SSO, με πρόθεμα το tenant id. **/
    userId?: string
    /** Το email του χρήστη που άφησε το σχόλιο. **/
    commenterEmail?: string
    /** Το όνομα του χρήστη που εμφανίζεται στο widget σχολίων. Με SSO, μπορεί να είναι displayName. **/
    commenterName: string
    /** Ακατέργαστο κείμενο σχολίου. **/
    comment: string
    /** Κείμενο σχολίου μετά την επεξεργασία. **/
    commentHTML: string
    /** Εξωτερικό id σχολίου. **/
    externalId?: string
    /** Το id του γονικού σχολίου. **/
    parentId?: string | null
    /** Η ημερομηνία UTC όταν αφήθηκε το σχόλιο. **/
    date: UTC_ISO_DateString
    /** Συνδυασμένο karma (up - down) των ψήφων. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True αν ο χρήστης ήταν συνδεδεμένος όταν σχολίασε, ή αν επικύρωσε το σχόλιο, ή αν επικύρωσε τη συνεδρία του όταν γράφτηκε το σχόλιο. **/
    verified: boolean
    /** Η ημερομηνία κατά την οποία το σχόλιο επικυρώθηκε. **/
    verifiedDate?: number
    /** Αν ένας moderator σήμανε το σχόλιο ως ελεγμένο. **/
    reviewed: boolean
    /** Η τοποθεσία, ή η κωδικοποίηση base64, του avatar. Θα είναι base64 μόνο αν αυτή ήταν η τιμή που περάστηκε με SSO. **/
    avatarSrc?: string
    /** Το σχόλιο επισημάνθηκε χειροκίνητα ή αυτόματα ως spam; **/
    isSpam: boolean
    /** Το σχόλιο επισημάνθηκε αυτόματα ως spam; **/
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
    /** Οι @αναφορές που γράφτηκαν στο σχόλιο και αναλύθηκαν/επεξεργάστηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Το domain από το οποίο προέρχεται το σχόλιο. **/
    domain?: string
    /** Η προαιρετική λίστα με moderation group ids που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Το αντικείμενο Mentions του Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το id χρήστη. Για χρήστες SSO, θα έχει το tenant id ως πρόθεμα. **/
    id: string
    /** Το τελικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπου χρήστης αναφέρθηκε. user = λογαριασμός FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Αν ο χρήστης απενεργοποιήσει τις ειδοποιήσεις, αυτό θα παραμείνει αληθές. **/
    sent: boolean
}
[inline-code-end]

#### Μέθοδοι HTTP

Μπορείτε να ρυθμίσετε τη μέθοδο HTTP για κάθε τύπο συμβάντος webhook στο admin panel:

- **Create Event**: POST ή PUT (προεπιλογή: PUT)
- **Update Event**: POST ή PUT (προεπιλογή: PUT)
- **Delete Event**: DELETE, POST ή PUT (προεπιλογή: DELETE)

Εφόσον όλα τα αιτήματα περιέχουν ένα ID, οι λειτουργίες Create και Update είναι idempotent από προεπιλογή (PUT). Η επανάληψη του ίδιου αιτήματος Create ή Update δεν θα πρέπει να δημιουργεί διπλότυπα αντικείμενα στην πλευρά σας.

#### Κεφαλίδες Αιτήματος

Κάθε αίτημα webhook περιλαμβάνει τις ακόλουθες κεφαλίδες:

| Κεφαλίδα | Περιγραφή |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Το API Secret σας |
| `X-FastComments-Timestamp` | Unix χρονοσφραγίδα (δευτερόλεπτα) όταν υπογράφηκε το αίτημα |
| `X-FastComments-Signature` | Υπογραφή HMAC-SHA256 (`sha256=<hex>`) |

Δείτε [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) για πληροφορίες σχετικά με την επαλήθευση της υπογραφής HMAC.

---