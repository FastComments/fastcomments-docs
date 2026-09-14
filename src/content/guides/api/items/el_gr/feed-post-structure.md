Ένα αντικείμενο `FeedPost` αντιπροσωπεύει μια ανάρτηση σε ένα feed του FastComments. Ένα feed είναι μια ροή αναρτήσεων με τα δικά τους νήματα σχολίων, που αποδίδεται από το widget Feed. Κάθε ανάρτηση έχει έναν συγγραφέα, προαιρετικό πλούσιο περιεχόμενο, πολυμέσα και συνδέσμους, και μπορεί να έχει ετικέτες ώστε το feed να μπορεί να φιλτραριστεί.

Η δομή για το αντικείμενο `FeedPost` είναι η εξής:

[inline-code-attrs-start title = 'Δομή FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** ΜΟΝΟ-ΑΝΑΓΝΩΣΗ **/
    _id: string
    /** ΜΟΝΟ-ΑΝΑΓΝΩΣΗ **/
    tenantId: string
    title?: string
    /** Το αναγνωριστικό του χρήστη FastComments ή SSO που δημιούργησε την ανάρτηση. **/
    fromUserId?: string
    /** Συμπληρώνεται από τον χρήστη όταν δεν έχει οριστεί. **/
    fromUserDisplayName?: string | null
    /** ΜΟΝΟ-ΑΝΑΓΝΩΣΗ. Συμπληρώνεται από τον χρήστη. **/
    fromUserAvatar?: string | null
    /** Χρησιμοποιείται για φιλτράρισμα ενός feed. **/
    tags?: string[]
    /** Βάρος ταξινόμησης εντός ενός feed. Οι υψηλότερες τιμές ταξινομούνται πρώτες. **/
    weight?: number
    /** Ζευγάρια κλειδιού/τιμής ελεύθερης μορφής για δική σας χρήση. **/
    meta?: Record<string, string>
    /** Καθαρισμένο HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** ΜΟΝΟ-ΑΝΑΓΝΩΣΗ **/
    createdAt: string
    /** ΜΟΝΟ-ΑΝΑΓΝΩΣΗ. Τύπος αντίδρασης για καταμέτρηση. **/
    reacts?: Record<string, number>
    /** ΜΟΝΟ-ΑΝΑΓΝΩΣΗ **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Πού οδηγεί το στοιχείο πολυμέσων όταν γίνεται κλικ. **/
    linkUrl?: string
    /** Μία καταχώρηση ανά απόδοση. Το widget επιλέγει το καλύτερο. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Το κείμενο του συνδέσμου, π.χ. "Εγγραφείτε τώρα". **/
    text?: string
    /** Μια επικεφαλίδα που εμφανίζεται με το σύνδεσμο. **/
    title?: string
    /** Μια περιγραφή που εμφανίζεται με το σύνδεσμο. **/
    description?: string
    url?: string
}
[inline-code-end]

Σημειώσεις:

- Ορισμένα από αυτά τα πεδία είναι σημειωμένα `READONLY` - επιστρέφονται από το API αλλά δεν μπορούν να οριστούν.
- Τα σχόλια σε μια ανάρτηση είναι κανονικά σχόλια των οποίων το `urlId` είναι `post:` ακολουθούμενο από το `_id` της ανάρτησης. Χρησιμοποιήστε αυτήν την τιμή με το Comment API για να διαβάσετε ή να δημιουργήσετε σχόλια σε μια ανάρτηση.