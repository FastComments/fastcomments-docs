[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα ενημέρωσης ενός μεμονωμένου σχολίου.

Σημειώσεις:

- Αυτό το API μπορεί να ενημερώσει το widget σχολίων "ζωντανά" αν το επιθυμείτε (αυτό αυξάνει το βασικό `creditsCost` από `1` σε `2`).
  - Αυτό μπορεί να κάνει τη μετεγκατάσταση σχολίων μεταξύ σελίδων "ζωντανή" (αλλάζοντας το `urlId`).
  - Οι μετεγκαταστάσεις κοστίζουν επιπλέον `2` credits καθώς οι σελίδες προϋπολογίζονται και αυτό απαιτεί υψηλή CPU.
- Σε αντίθεση με το API δημιουργίας, αυτό το API ΔΕΝ θα δημιουργεί αυτόματα αντικείμενα χρήστη στο σύστημά μας αν παρέχεται email.
- Τα σχόλια που ενημερώνονται μέσω αυτού του API μπορούν ακόμη να ελεγχθούν για spam, αν το επιθυμείτε.
- Ρυθμίσεις όπως το μέγιστο μήκος σχολίου, εάν έχουν ρυθμιστεί μέσω της σελίδας διαχείρισης Customization Rule, θα εφαρμόζονται εδώ.
- Για να επιτρέψετε στους χρήστες να ενημερώσουν το κείμενο του σχολίου τους, μπορείτε απλά να καθορίσετε το `comment` στο σώμα του αιτήματος. Εμείς θα δημιουργήσουμε το προκύπτον `commentHTML`.
  - Εάν καθορίσετε και τα `comment` και `commentHTML`, δεν θα δημιουργήσουμε αυτόματα το HTML.
  - Εάν ο χρήστης προσθέσει mentions ή hashtags στο νέο κείμενό του, θα επεξεργαστείται όπως και στο `POST` API.
- Όταν ενημερώνετε το `commenterEmail` σε ένα σχόλιο, είναι καλύτερο επίσης να καθορίσετε το `userId`. Διαφορετικά, πρέπει να διασφαλίσετε ότι ο χρήστης με αυτό το email ανήκει στον tenant σας, αλλιώς το αίτημα θα αποτύχει.  
- Εάν το στοχευόμενο σχόλιο είναι κλειδωμένο (`isLocked: true`), το αίτημα απορρίπτεται με `code: 'locked'`. Ξεκλειδώστε πρώτα το σχόλιο, ενημερώστε το, και μετά κλειδώστε το ξανά αν το επιθυμείτε.


[inline-code-attrs-start title = 'Ελάχιστο Παράδειγμα PATCH cURL Σχολίου'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος PATCH Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Ο χρήστης που πραγματοποιεί την ενημέρωση. Αν επιθυμείτε, μπορεί να χρησιμοποιηθεί για να ελεγχθεί εάν μπορούν να επεξεργαστούν το σχόλιο.  **/
    contextUserId?: string
	/** Να ελέγξουμε αν το νέο σχόλιο μοιάζει με spam;  **/
    doSpamCheck?: 'true' | 'false'
	/** Εάν το σχόλιο πρέπει να εμφανίζεται "ζωντανά" στους χρήστες που βλέπουν περιπτώσεις του widget σχολίων με το ίδιο urlId. ΣΗΜΕΙΩΣΗ: Διπλασιάζει το κόστος σε credits από 1 σε 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης PATCH Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
}
[inline-code-end]