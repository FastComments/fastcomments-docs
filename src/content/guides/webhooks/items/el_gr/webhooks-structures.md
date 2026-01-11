Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, που περιγράφεται σε TypeScript παρακάτω.

#### Η Δομή του Αντικειμένου WebhookComment

##### Η Δομή του Γεγονότος "Create"
Το σώμα του αιτήματος για το γεγονός "create" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του Γεγονότος "Update"
Το σώμα του αιτήματος για το γεγονός "update" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του Γεγονότος "Delete"
Το σώμα του αιτήματος για το γεγονός "delete" είναι ένα αντικείμενο WebhookComment.

    Αλλαγή από Nov 14th 2023
    Προηγουμένως το σώμα του αιτήματος για το γεγονός "delete" περιείχε μόνο το id του σχολίου. Τώρα περιέχει ολόκληρο το σχόλιο τη στιγμή της διαγραφής.


[inline-code-attrs-start title = 'Το Αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Το id του σχολίου. **/
    id: string
    /** Το id ή το URL που προσδιορίζει το νήμα σχολίων. Κανονικοποιημένο. **/
    urlId: string
    /** Το URL που δείχνει το σημείο όπου έγινε το σχόλιο. **/
    url?: string
    /** Το id του χρήστη που έκανε το σχόλιο. Εάν SSO, προθεματισμένο με tenant id. **/
    userId?: string
    /** Το email του χρήστη που άφησε το σχόλιο. **/
    commenterEmail?: string
    /** Το όνομα του χρήστη που εμφανίζεται στο widget σχολίων. Σε SSO, μπορεί να είναι displayName. **/
    commenterName: string
    /** Ακατέργαστο κείμενο σχολίου. **/
    comment: string
    /** Το κείμενο του σχολίου μετά την επεξεργασία/ανάλυση. **/
    commentHTML: string
    /** Εξωτερικό id του σχολίου. **/
    externalId?: string
    /** Το id του γονικού σχολίου. **/
    parentId?: string | null
    /** Η ημερομηνία UTC όταν καταχωρήθηκε το σχόλιο. **/
    date: UTC_ISO_DateString
    /** Συνδυασμένο karma (up - down) των ψήφων. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True εάν ο χρήστης ήταν συνδεδεμένος όταν σχολίασε, ή αν έχει επαληθευτεί το σχόλιο, ή αν επαλήθευσε τη συνεδρία του όταν καταχωρήθηκε το σχόλιο. **/
    verified: boolean
    /** Ημερομηνία επαλήθευσης του σχολίου. **/
    verifiedDate?: number
    /** Αν ένας moderator σημείωσε το σχόλιο ως αναθεωρημένο. **/
    reviewed: boolean
    /** Η τοποθεσία ή η base64 κωδικοποίηση του avatar. Θα είναι base64 μόνο εάν αυτή ήταν η τιμή που δόθηκε με SSO. **/
    avatarSrc?: string
    /** Το σχόλιο επισημάνθηκε ως ανεπιθύμητο (spam) χειροκίνητα ή αυτόματα; **/
    isSpam: boolean
    /** Το σχόλιο επισημάνθηκε αυτόματα ως ανεπιθύμητο (spam); **/
    aiDeterminedSpam: boolean
    /** Υπάρχουν εικόνες στο σχόλιο; **/
    hasImages: boolean
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για τη σειρά ταξινόμησης "Most Relevant". **/
    pageNumber: number
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για τη σειρά ταξινόμησης "Oldest First". **/
    pageNumberOF: number
    /** Ο αριθμός σελίδας στον οποίο βρίσκεται το σχόλιο για τη σειρά ταξινόμησης "Newest First". **/
    pageNumberNF: number
    /** Το σχόλιο εγκρίθηκε αυτόματα ή χειροκίνητα; **/
    approved: boolean
    /** Ο κωδικός τοπικής ρύθμισης (format: en_us) του χρήστη όταν γράφτηκε το σχόλιο. **/
    locale: string
    /** Οι @mentions που γράφτηκαν στο σχόλιο και αναλύθηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Το domain από το οποίο προέρχεται το σχόλιο. **/
    domain?: string
    /** Η προαιρετική λίστα με τα moderation group ids που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Όταν χρήστες ετικετοποιούνται σε ένα σχόλιο, οι πληροφορίες αποθηκεύονται σε μια λίστα που ονομάζεται `mentions`. Κάθε αντικείμενο σε αυτή τη λίστα
έχει την ακόλουθη δομή.

[inline-code-attrs-start title = 'Το Αντικείμενο Αναφορών Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το id του χρήστη. Για χρήστες SSO, θα έχει προθεματισμένο το tenant id. **/
    id: string
    /** Το τελικό κείμενο της ετικέτας @mention, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό κείμενο της ετικέτας @mention, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπο χρήστη ετικετοποιήθηκε. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ακόμη και αν ο χρήστης απενεργοποιήσει τις ειδοποιήσεις, αυτό θα παραμείνει true. **/
    sent: boolean
}
[inline-code-end]

#### Χρήση HTTP Μεθόδων

**Τα Create και Update χρησιμοποιούν HTTP PUT και όχι POST!**

Εφόσον όλα τα αιτήματά μας περιέχουν ένα ID, η επανάληψη του ίδιου αιτήματος Create ή Update δεν θα πρέπει να δημιουργήσει νέα αντικείμενα στην πλευρά σας.

Αυτό σημαίνει ότι αυτές οι κλήσεις είναι idempotent και θα πρέπει να είναι γεγονότα τύπου PUT σύμφωνα με τις προδιαγραφές HTTP.