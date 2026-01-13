Το FastComments παρέχει μια εύκολη στη χρήση λύση SSO. Η ενημέρωση των πληροφοριών ενός χρήστη με την ενσωμάτωση βασισμένη σε HMAC είναι
τόσο απλή όσο η φόρτωση της σελίδας από τον χρήστη με ένα ενημερωμένο φορτίο.

Ωστόσο, μπορεί να είναι επιθυμητό να διαχειριστείτε έναν χρήστη εκτός αυτής της ροής, για να βελτιώσετε τη συνέπεια της εφαρμογής σας.

Το API SSO User παρέχει έναν τρόπο για CRUD αντικειμένων που ονομάζουμε SSOUsers. Αυτά τα αντικείμενα είναι διαφορετικά από τους κανονικούς Users και
διατηρούνται χωριστά για ασφάλεια τύπου.

Η δομή για το αντικείμενο SSOUser είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. **/
        badgeIds: string[]
        /** If true, replaces all existing displayed badges with the provided ones. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### Χρέωση για Χρήστες SSO

Οι χρήστες SSO χρεώνονται διαφορετικά με βάση τις σημαίες δικαιωμάτων τους:

- **Κανονικοί Χρήστες SSO**: Χρήστες χωρίς δικαιώματα διαχειριστή ή συντονιστή χρεώνονται ως κανονικοί χρήστες SSO
- **Διαχειριστές SSO**: Χρήστες με σημαίες `isAccountOwner` ή `isAdminAdmin` χρεώνονται ξεχωριστά ως Διαχειριστές SSO (ίδια τιμή με τους κανονικούς διαχειριστές ενοικιαστή)
- **Συντονιστές SSO**: Χρήστες με σημαία `isCommentModeratorAdmin` χρεώνονται ξεχωριστά ως Συντονιστές SSO (ίδια τιμή με τους κανονικούς συντονιστές)

**Σημαντικό**: Για να αποφευχθεί η διπλή χρέωση, το σύστημα αφαιρεί αυτόματα τα διπλότυπα χρηστών SSO έναντι κανονικών χρηστών ενοικιαστή και συντονιστών με βάση τη διεύθυνση email. Αν ένας χρήστης SSO έχει το ίδιο email με έναν κανονικό χρήστη ενοικιαστή ή συντονιστή, δεν θα χρεωθεί δύο φορές.

### Έλεγχος Πρόσβασης

Οι χρήστες μπορούν να χωριστούν σε ομάδες. Αυτό είναι για το πεδίο `groupIds`, και είναι προαιρετικό.

### @Αναφορές

Από προεπιλογή οι `@αναφορές` θα χρησιμοποιούν το `username` για αναζήτηση άλλων χρηστών sso όταν πληκτρολογηθεί ο χαρακτήρας `@`. Αν χρησιμοποιείται το `displayName`, τότε τα αποτελέσματα που ταιριάζουν με
το `username` θα αγνοηθούν όταν υπάρχει αντιστοιχία για το `displayName`, και τα αποτελέσματα αναζήτησης `@αναφοράς` θα χρησιμοποιούν το `displayName`.

### Συνδρομές

Με το FastComments, οι χρήστες μπορούν να εγγραφούν σε μια σελίδα κάνοντας κλικ στο εικονίδιο καμπάνας στο widget σχολίων και κάνοντας κλικ στο Εγγραφή.

Με έναν κανονικό χρήστη, τους στέλνουμε emails ειδοποίησης με βάση τις ρυθμίσεις ειδοποιήσεων τους.

Με τους Χρήστες SSO, το χωρίζουμε αυτό για συμβατότητα προς τα πίσω. Οι χρήστες θα λαμβάνουν αυτά τα επιπλέον emails ειδοποίησης συνδρομής
μόνο αν ορίσετε το `optedInSubscriptionNotifications` σε `true`.

### Εμβλήματα

Μπορείτε να αναθέσετε εμβλήματα σε χρήστες SSO χρησιμοποιώντας την ιδιότητα `badgeConfig`. Τα εμβλήματα είναι οπτικοί δείκτες που εμφανίζονται δίπλα στο όνομα ενός χρήστη στα σχόλια.

- `badgeIds` - Ένας πίνακας με IDs εμβλημάτων για ανάθεση στον χρήστη. Αυτά πρέπει να είναι έγκυρα IDs εμβλημάτων που δημιουργήθηκαν στον λογαριασμό FastComments σας. Περιορισμένο σε 30 εμβλήματα.
- `override` - Αν είναι αληθές, όλα τα υπάρχοντα εμβλήματα που εμφανίζονται στα σχόλια θα αντικατασταθούν με τα παρεχόμενα. Αν είναι ψευδές ή παραλείπεται, τα παρεχόμενα εμβλήματα θα προστεθούν σε οποιαδήποτε υπάρχοντα εμβλήματα.
- `update` - Αν είναι αληθές, οι ιδιότητες εμφάνισης εμβλήματος θα ενημερωθούν από τη διαμόρφωση ενοικιαστή όποτε ο χρήστης συνδέεται.
