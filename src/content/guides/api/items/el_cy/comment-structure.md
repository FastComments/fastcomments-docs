A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`.

The structure for the Comment object is as follows:

[inline-code-attrs-start title = 'Δομή του Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Ορισμένο σε true εάν η μηχανή spam προσδιόρισε ότι το σχόλιο ήταν spam. **/
    aiDeterminedSpam?: boolean
    /** Εάν το σχόλιο είναι εγκεκριμένο για εμφάνιση. Ορίζεται σε true κατά την αποθήκευση του σχολίου, διαφορετικά θα είναι κρυφό. **/
    approved?: boolean
    /** Το avatar του χρήστη. **/
    avatarSrc?: string
    /** Παιδικά σχόλια. Δεν συμπληρώνεται σε όλες τις περιπτώσεις. Χρησιμοποιείται όταν asTree οριστεί σε true μέσω του API. **/
    children: Comment[]
    /** Το ακατέργαστο σχόλιο του σχολιαστή. **/
    comment: string
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο του σχολιαστή αναλυμένο σε HTML. **/
    commentHTML?: string
    /** Το email του σχολιαστή. Απαιτείται εάν τα ανώνυμα σχόλια είναι απενεργοποιημένα. **/
    commenterEmail?: string
    /** Ο σύνδεσμος του σχολιαστή (π.χ. το blog του). **/
    commenterLink?: string
    /** Το όνομα του σχολιαστή. Πάντα απαιτείται. Αν δεν υπάρχει, ορίστε κάτι σαν "Ανώνυμος". **/
    commenterName: string
    /** Η ημερομηνία που το σχόλιο καταχώρηθηκε, σε UTC epoch. **/
    date: number
    /** Η "ετικέτα εμφάνισης" για το σχόλιο - για παράδειγμα "Admin", "Moderator", ή κάτι σαν "VIP User". **/
    displayLabel?: string
    /** Το domain όπου δημοσιεύτηκε το σχόλιο. **/
    domain?: string
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Ο αριθμός φορών που το σχόλιο επισημάνθηκε. **/
    flagCount?: number
    /** Τα #hashtags που γράφτηκαν στο σχόλιο και αναλύθηκαν επιτυχώς. Μπορείτε επίσης να προσθέσετε χειροκίνητα hashtags, για ερωτήματα, αλλά δεν θα εμφανιστούν αυτόματα στο κείμενο του σχολίου. **/
    hashTags?: CommentHashTag[]
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο περιέχει εικόνες; **/
    hasImages?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο περιέχει συνδέσμους; **/
    hasLinks?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το μοναδικό id του σχολίου. **/
    id: string
    /** Μόνο κατά τη δημιουργία! Αυτό κατακερματίζεται (hashed) για αποθήκευση. **/
    ip?: string
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Ο τρέχων χρήστης απέκλεισε το χρήστη που έγραψε αυτό το σχόλιο; **/
    isBlocked?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο είναι από admin; Ορίζεται αυτόματα με βάση το userId. **/
    isByAdmin?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο είναι από moderator; Ορίζεται αυτόματα με βάση το userId. **/
    isByModerator?: boolean
    /** Ορίζεται σε true αν το σχόλιο διαγράφηκε μαλακά (soft deleted) (έπρεπε να μείνει placeholder λόγω κάποιας άλλης ρύθμισης). **/
    isDeleted?: boolean
    /** Ορίζεται σε true αν ο λογαριασμός του χρήστη διαγράφηκε και το σχόλιο έπρεπε να διατηρηθεί. **/
    isDeletedUser?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Έχει γίνει flag από τον τρέχοντα συνδεδεμένο χρήστη (contextUserId); **/
    isFlagged?: boolean
    /** Είναι το σχόλιο καρφιτσωμένο (pinned); **/
    isPinned?: boolean
    /** Είναι το σχόλιο κλειδωμένο; Όταν είναι true, κανείς (συμπεριλαμβανομένων των moderators) δεν μπορεί να απαντήσει, να επεξεργαστεί ή να διαγράψει μέχρι να ξεκλειδωθεί. **/
    isLocked?: boolean
    /** Είναι το σχόλιο spam; **/
    isSpam?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο έχει ψηφιστεί αρνητικά από τον τρέχοντα χρήστη (contextUserId); **/
    isVotedDown?: boolean
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το σχόλιο έχει ψηφιστεί θετικά από τον τρέχοντα χρήστη (contextUserId); **/
    isVotedUp?: boolean
    /** Η τοπική ρύθμιση (locale) του σχολίου. Αν δεν παρέχεται, θα προέλθει από το HTTP Accept-Language header. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Τα @mentions που γράφτηκαν στο σχόλιο και αναλύθηκαν επιτυχώς. **/
    mentions?: CommentUserMention[]
    /** Προαιρετικά metadata που σχετίζονται με το σχόλιο. **/
    meta?: Record<string, string | number | boolean>
    /** Η προαιρετική λίστα με τα ids ομάδων moderation που σχετίζονται με αυτό το σχόλιο. **/
    moderationGroupIds?: string[]|null
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το id του αντικειμένου ψήφου που αντιστοιχεί στην ψήφο του τρέχοντος χρήστη (contextUserId) για αυτό το σχόλιο. **/
    myVoteId?: string
    /** Εάν εστάλησαν ειδοποιήσεις για αυτό το σχόλιο προς τους σχολιαστές. Για να αποτρέψετε την αποστολή ειδοποιήσεων κατά τις εισαγωγές, ορίστε αυτό σε true. **/
    notificationSentForParent?: boolean
    /** Εάν εστάλησαν ειδοποιήσεις για αυτό το σχόλιο προς χρήστες του tenant. Για να αποτρέψετε την αποστολή ειδοποιήσεων κατά τις εισαγωγές, ορίστε αυτό σε true. **/
    notificationSentForParentTenant?: boolean
    /** Ο τίτλος της σελίδας στην οποία ήταν αυτό το σχόλιο. **/
    pageTitle?: string
    /** Εάν απαντάμε σε σχόλιο, αυτό είναι το ID στο οποίο απαντάμε. **/
    parentId?: string|null
    /** Εάν το σχόλιο έχει σημειωθεί ως ελεγμένο (reviewed). **/
    reviewed: boolean
    /** Το tenant id στο οποίο ανήκει το σχόλιο. **/
    tenantId: string
    /** Ο χρήστης που έγραψε το σχόλιο. Δημιουργείται αυτόματα όταν αποθηκεύεται ένα σχόλιο με όνομα/email. **/
    userId?: string|null
    /** Το URL στη θέση όπου είναι ορατό αυτό το σχόλιο, όπως μια ανάρτηση blog. **/
    url: string
    /** Μια "καθαρισμένη" έκδοση του urlId που μας δώσατε. Κατά την αποθήκευση, προσδιορίζετε αυτό το πεδίο, αλλά όταν ανακτάτε το σχόλιο πίσω αυτό θα "καθαριστεί" και η αρχική σας τιμή θα μετακινηθεί σε "urlIdRaw". **/
    urlId: string
    /** ΜΟΝΟ ΑΝΑΓΝΩΣΗ: Το αρχικό urlId που μας δώσατε. **/
    urlIdRaw?: string
    /** Είναι ο χρήστης και αυτό το σχόλιο επαληθευμένα; **/
    verified: boolean
    /** Αριθμός θετικών ψήφων. **/
    votesUp?: number
    /** Αριθμός αρνητικών ψήφων. **/
    votesDown?: number
    /** Η "karma" του σχολίου (= ψήφοι υπέρ - ψήφοι κατά). **/
    votes?: number
}
[inline-code-end]

Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.

### Δομή Κειμένου Σχολίου

Comments are written in a FastComments flavor of markdown, which is just markdown plus traditional `bbcode` style tags for images, like `[img]path[/img]`.

Text is stored in two fields. The text the user entered is stored unmodified in the `comment` field. This is rendered and stored in the `commentHTML` field.

The allowed HTML tags are `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

It's recommended to render the HTML, since it is a very small subset of HTML, building a renderer is pretty straightforward. There are multiple libraries for React Native and Flutter, for instance, to help with this

You may choose to render the un-normalized value of the `comment` field. [An example parser is here.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

The example parser could also be adjusted to work with HTML, and transform the HTML tags into expected elements to render for your platform. 

### Επισήμανση

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Το αντικείμενο Comment Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Το id του χρήστη. Για χρήστες SSO, αυτό θα έχει ως πρόθεμα το tenant id σας. **/
    id: string
    /** Το τελικό @mention tag text, συμπεριλαμβανομένου του συμβόλου @. **/
    tag: string
    /** Το αρχικό @mention tag text, συμπεριλαμβανομένου του συμβόλου @. **/
    rawTag: string
    /** Τι τύπος χρήστη επισημάνθηκε. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ακόμα και αν ο χρήστης εξαιρεθεί από τις ειδοποιήσεις, αυτό θα εξακολουθεί να ορίζεται σε true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

When hashtags are used and successfully parsed, the information is stored in a list called `hashTags`. Each object in that list
has the following structure. Hashtags can also be manually added to the comment `hashTags` array for querying, if `retain` is set.

[inline-code-attrs-start title = 'Το αντικείμενο Comment HashTag'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Το id του hashtag. **/
    id: string
    /** Το τελικό #hashtag tag text, συμπεριλαμβανομένου του συμβόλου #. **/
    tag: string
    /** Εάν το hashtag συνδέεται με ένα προσαρμοσμένο URL, αυτό θα οριστεί. **/
    url?: string
    /** Εάν πρέπει να διατηρήσουμε το hashtag, ακόμη κι αν δεν υπάρχει στο κείμενο του σχολίου, όταν το σχόλιο ενημερώνεται. Χρήσιμο για επισημάνσεις σχολίων χωρίς να αλλάζει το κείμενο του σχολίου. **/
    retain?: boolean
}
[inline-code-end]

---