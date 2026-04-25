[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το endpoint API παρέχει τη δυνατότητα διαγραφής ενός σχολίου.

Σημειώσεις:

- Αυτό το API μπορεί να ενημερώσει το widget σχολίων "live" εάν το επιθυμείτε (αυτό αυξάνει `creditsCost` από `1` σε `2`).
- Αυτό το API θα διαγράψει όλα τα παιδικά σχόλια.
- Εάν το στοχευόμενο σχόλιο είναι κλειδωμένο (`isLocked: true`), το αίτημα απορρίπτεται με `code: 'locked'`. Ξεκλειδώστε πρώτα το σχόλιο, και μετά διαγράψτε το.

[inline-code-attrs-start title = 'Παράδειγμα cURL DELETE Σχολίου'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Δομή Αιτήματος DELETE Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Ο χρήστης που πραγματοποιεί την ενημέρωση. Εάν το επιθυμείτε, μπορεί να χρησιμοποιηθεί για να ελεγχθεί αν μπορεί να διαγράψει το σχόλιο.  **/
    contextUserId?: string
	/** Εάν το σχόλιο θα πρέπει να διαγραφεί "live" για χρήστες που βλέπουν εμφανίσεις του widget σχολίων με το ίδιο urlId. ΣΗΜΕΙΩΣΗ: Διπλασιάζει το κόστος σε credits από 1 σε 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης DELETE Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
}
[inline-code-end]