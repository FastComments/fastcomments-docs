[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα ενημέρωσης ενός μόνο σχολίου.

Σημειώσεις:

- Αυτό το API μπορεί να ενημερώσει το comment widget "live" αν επιθυμείτε (αυτό αυξάνει το βασικό `creditsCost` από `1` σε `2`).
  - Αυτό μπορεί να κάνει τη μετεγκατάσταση σχολίων μεταξύ σελίδων "live" (αλλαγή `urlId`).
  - Οι μετεγκαταστάσεις κοστίζουν επιπλέον `2` credits καθώς οι σελίδες προϋπολογίζονται και αυτό είναι εντατικό σε CPU.
- Σε αντίθεση με το create API, αυτό το API ΔΕΝ θα δημιουργήσει αυτόματα αντικείμενα χρηστών στο σύστημά μας αν παρέχεται email.
- Τα σχόλια που ενημερώνονται μέσω αυτού του API μπορούν ακόμα να ελεγχθούν για spam αν επιθυμείτε.
- Διαμόρφωση όπως μέγιστο μήκος σχολίου, αν διαμορφωθεί μέσω της σελίδας διαχείρισης Customization Rule, θα εφαρμοστεί εδώ.
- Για να επιτρέψετε στους χρήστες να ενημερώσουν το κείμενο του σχολίου τους, μπορείτε απλά να καθορίσετε `comment` στο request body. Θα δημιουργήσουμε το αποτέλεσμα `commentHTML`.
  - Αν ορίσετε και `comment` και `commentHTML` δεν θα δημιουργήσουμε αυτόματα το HTML.
  - Αν ο χρήστης προσθέσει mentions ή hashtags στο νέο κείμενό του, θα επεξεργαστεί ακόμα όπως το `POST` API.
- Όταν ενημερώνετε το `commenterEmail` σε ένα σχόλιο, είναι καλύτερο να καθορίσετε επίσης `userId`. Διαφορετικά, πρέπει να διασφαλίσετε ότι ο χρήστης με αυτό το email ανήκει στο tenant σας, αλλιώς το αίτημα θα αποτύχει.


[inline-code-attrs-start title = 'Ελάχιστη Ενημέρωση Σχολίου PATCH cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Ενημέρωσης Σχολίου PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Ενημέρωσης Σχολίου PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
