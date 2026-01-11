Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, περιγραμμένο σε TypeScript παρακάτω.

#### Η Δομή του Αντικειμένου WebhookComment

##### Η Δομή του Event "Create"
Το request body του event "create" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του Event "Update"
Το request body του event "update" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του Event "Delete"
Το request body του event "delete" είναι ένα αντικείμενο WebhookComment.

    Αλλαγή από 14 Νοεμβρίου 2023
    Προηγουμένως, το request body του event "delete" περιείχε μόνο το id του σχολίου. Τώρα περιέχει το πλήρες σχόλιο όπως ήταν κατά τη στιγμή της διαγραφής.


[inline-code-attrs-start title = 'Το Αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Το id του σχολίου. **/
    id: string
    /** Το id ή το URL που ταυτοποιεί το νήμα σχολίων. Κανονικοποιημένο. **/
    urlId: string
    /** Το URL που δείχνει το σημείο όπου αφήθηκε το σχόλιο. **/
    url?: string
    /** Το id χρήστη που άφησε το σχόλιο. Αν είναι SSO, έχει προθεματοποιηθεί με το tenant id. **/
    userId?: string
    /** Το email του χρήστη που άφησε το σχόλιο. **/
    commenterEmail?: string
    /** Το όνομα του χρήστη που εμφανίζεται στο widget σχολίων. Με SSO, μπορεί να είναι displayName. **/
    commenterName: string
    /** Ακατέργαστο κείμενο του σχολίου. **/
    comment: string
    /** Κείμενο σχολίου μετά την ανάλυση. **/
    commentHTML: string
    /** Εξωτερικό id σχολίου. **/
    externalId?: string
    /** Το id του γονικού σχολίου. **/
    parentId?: string | null
    /** Η ημερομηνία UTC όταν γράφτηκε το σχόλιο. **/
    date: UTC_ISO_DateString
    /** Συνολική karma (up - down) των ψήφων. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True αν ο χρήστης ήταν συνδεδεμένος όταν σχολίασε, ή αν επαλήθευσε το σχόλιο, ή αν επαλήθευσε τη συνεδρία του όταν γράφτηκε το σχόλιο. **/
    verified: boolean
    /** Ημερομηνία επαλήθευσης του σχολίου. **/
    verifiedDate?: number
    /** Αν ένας moderator σήμανε το σχόλιο ως ελεγμένο. **/
    reviewed: boolean
    /** Η τοποθεσία, ή η κωδικοποίηση base64, του avatar. Θα είναι base64 μόνο αν αυτή ήταν η τιμή που περάστηκε με SSO. **/
    avatarSrc?: string
    /** Το σχόλιο σημειώθηκε χειροκίνητα ή αυτόματα ως spam; **/
    isSpam: boolean
    /** Το σχόλιο σημειώθηκε αυτόματα ως spam; **/
    aiDeterminedSpam: boolean
    /** Υπάρχουν εικόνες στο σχόλιο; **/
    hasImages: boolean
    /** Αριθμός σελίδας όπου βρίσκεται το σχόλιο για την ταξινόμηση "Most Relevant". **/
    pageNumber: number
    /** Αριθμός σελίδας όπου βρίσκεται το σχόλιο για την ταξινόμηση "Oldest First". **/
    pageNumberOF: number
    /** Αριθμός σελίδας όπου βρίσκεται το σχόλιο για την ταξινόμηση "Newest First". **/
    pageNumberNF: number
    /** Το σχόλιο εγκρίθηκε αυτόματα ή χειροκίνητα; **/
    approved: boolean
    /** Ο κωδικός locale (μορφή: en_us) του χρήστη όταν γράφτηκε το σχόλιο. **/
    locale: string
    /** Οι @mentions που γράφτηκαν στο σχόλιο και αναλύθηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Το domain από το οποίο προέρχεται το σχόλιο. **/
    domain?: string
    /** Η προαιρετική λίστα με ids ομάδων moderation που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Όταν χρήστες εσημειώνονται σε ένα σχόλιο, οι πληροφορίες αποθηκεύονται σε μία λίστα που ονομάζεται `mentions`. Κάθε αντικείμενο σε αυτή τη λίστα
έχει την ακόλουθη δομή.

[inline-code-attrs-start title = 'Το Αντικείμενο Mentions του Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το id του χρήστη. Για χρήστες SSO, αυτό θα έχει προθεματοποιημένο το tenant id. **/
    id: string
    /** Το τελικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό κείμενο του @mention tag, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπος χρήστη εσήμανθηκε. user = λογαριασμός FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ακόμα κι αν ο χρήστης επιλέξει να μην λαμβάνει ειδοποιήσεις, αυτό θα παραμείνει true. **/
    sent: boolean
}
[inline-code-end]

#### Χρησιμοποιούμενες Μέθοδοι HTTP

**Τα Create και Update χρησιμοποιούν HTTP PUT και όχι POST!**

Εφόσον όλα τα requests μας περιέχουν ένα ID, η επανάληψη του ίδιου Create ή Update request δεν θα πρέπει να δημιουργεί νέα αντικείμενα στην πλευρά σας.

Αυτό σημαίνει ότι αυτές οι κλήσεις είναι idempotent και πρέπει να είναι PUT events σύμφωνα με τις προδιαγραφές HTTP.