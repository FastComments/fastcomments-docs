Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, που περιγράφεται σε TypeScript παρακάτω.

#### Η Δομή του Αντικειμένου WebhookComment

##### The "Create" Event Structure
Το σώμα του αιτήματος για το event "create" είναι ένα αντικείμενο WebhookComment.

##### The "Update" Event Structure
Το σώμα του αιτήματος για το event "update" είναι ένα αντικείμενο WebhookComment.

##### The "Delete" Event Structure
Το σώμα του αιτήματος για το event "delete" είναι ένα αντικείμενο WebhookComment.

    Αλλαγή από 14 Νοεμβρίου 2023
    Προηγουμένως, το σώμα του αιτήματος για το event "delete" περιείχε μόνο το id του σχολίου. Τώρα περιέχει ολόκληρο το σχόλιο τη στιγμή της διαγραφής.


[inline-code-attrs-start title = 'Το Αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Το id του σχολίου. **/
    id: string
    /** Το id ή το URL που προσδιορίζει το νήμα σχολίων. Κανονικοποιημένο. **/
    urlId: string
    /** Το URL που δείχνει πού γράφτηκε το σχόλιο. **/
    url?: string
    /** Το user id που άφησε το σχόλιο. Αν είναι SSO, έχει προθεματισμένο το tenant id. **/
    userId?: string
    /** Το email του χρήστη που άφησε το σχόλιο. **/
    commenterEmail?: string
    /** Το όνομα του χρήστη που εμφανίζεται στο widget σχολίων. Με SSO, μπορεί να είναι το displayName. **/
    commenterName: string
    /** Ακατέργαστο κείμενο σχολίου. **/
    comment: string
    /** Κείμενο σχολίου μετά την ανάλυση. **/
    commentHTML: string
    /** Εξωτερικό id σχολίου. **/
    externalId?: string
    /** Το id του γονικού σχολίου. **/
    parentId?: string | null
    /** Η ημερομηνία UTC όταν το σχόλιο καταχωρήθηκε. **/
    date: UTC_ISO_DateString
    /** Συνδυασμένο karma (up - down) των ψήφων. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True αν ο χρήστης ήταν συνδεδεμένος όταν σχολίασε, ή αν επαλήθευσε το σχόλιο, ή αν επαλήθευσε τη συνεδρία του όταν το σχόλιο γράφτηκε. **/
    verified: boolean
    /** Ημερομηνία που το σχόλιο επαληθεύτηκε. **/
    verifiedDate?: number
    /** Αν ένας moderator σήμανε το σχόλιο ως αναθεωρημένο. **/
    reviewed: boolean
    /** Η τοποθεσία, ή η base64 κωδικοποίηση, του avatar. Θα είναι base64 μόνο αν αυτή ήταν η τιμή που στάλθηκε με SSO. **/
    avatarSrc?: string
    /** Το σχόλιο σημαδεύτηκε χειροκίνητα ή αυτόματα ως spam; **/
    isSpam: boolean
    /** Το σχόλιο χαρακτηρίστηκε αυτόματα ως spam; **/
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
    /** Ο κωδικός locale (μορφή: en_us) του χρήστη όταν το σχόλιο γράφτηκε. **/
    locale: string
    /** Οι @mentions που γράφτηκαν στο σχόλιο και επεξεργάστηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Το domain από το οποίο προέρχεται το σχόλιο. **/
    domain?: string
    /** Η προαιρετική λίστα με moderation group ids που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Το Αντικείμενο Mentions του Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το user id. Για χρήστες SSO, αυτό θα έχει προθεματισμένο το tenant id σας. **/
    id: string
    /** Το τελικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπος χρήστη επισημάνθηκε. user = λογαριασμός FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ακόμη και αν ο χρήστης έχει απενεργοποιήσει τις ειδοποιήσεις, αυτό θα παραμένει true. **/
    sent: boolean
}
[inline-code-end]

#### Μέθοδοι HTTP

Μπορείτε να ρυθμίσετε τη μέθοδο HTTP για κάθε τύπο event webhook στο admin panel:

- **Create Event**: POST ή PUT (προεπιλογή: PUT)
- **Update Event**: POST ή PUT (προεπιλογή: PUT)
- **Delete Event**: DELETE, POST ή PUT (προεπιλογή: DELETE)

Εφόσον όλα τα αιτήματα περιέχουν ένα ID, οι λειτουργίες Create και Update είναι idempotent από προεπιλογή (PUT). Η επανάληψη του ίδιου αιτήματος Create ή Update δεν θα πρέπει να δημιουργεί διπλότυπα αντικείμενα στην πλευρά σας.

#### Επικεφαλίδες Αιτήματος

Κάθε αίτημα webhook περιλαμβάνει τις ακόλουθες επικεφαλίδες:

| Επικεφαλίδα | Περιγραφή |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Το API Secret σας |
| `X-FastComments-Timestamp` | Unix timestamp (σε δευτερόλεπτα) όταν υπογράφηκε το αίτημα |
| `X-FastComments-Signature` | Υπογραφή HMAC-SHA256 (`sha256=<hex>`) |

Δείτε [Ασφάλεια & API Tokens](/guides/webhooks/webhooks-api-tokens) για πληροφορίες σχετικά με την επαλήθευση της υπογραφής HMAC.