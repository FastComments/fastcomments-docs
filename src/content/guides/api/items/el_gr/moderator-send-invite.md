[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα πρόσκλησης ενός μόνο `Moderator`.

Οι ακόλουθοι περιορισμοί υπάρχουν για την αποστολή email πρόσκλησης σε έναν `Moderator`:
- Ο `Moderator` πρέπει να υπάρχει ήδη.
- Το `fromName` δεν μπορεί να είναι μεγαλύτερο από `100 χαρακτήρες`.

**Σημειώσεις:**
- Αν υπάρχει ήδη χρήστης με το παρεχόμενο email, θα προσκληθεί να συντονίσει τα σχόλια του ενοικιαστή σας.
- Αν **δεν υπάρχει** χρήστης με το παρεχόμενο email, ο σύνδεσμος πρόσκλησης θα τον καθοδηγήσει στη δημιουργία του λογαριασμού του.
- Η πρόσκληση θα λήξει μετά από `30 ημέρες`.

Μπορούμε να δημιουργήσουμε έναν `Moderator` για έναν χρήστη για τον οποίο γνωρίζουμε μόνο το email:

[inline-code-attrs-start title = 'Πρόσκληση Συντονιστή cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Αυτό θα στείλει ένα email όπως `Ο Bob στο TenantName σας προσκαλεί να γίνετε συντονιστής...`

[inline-code-attrs-start title = 'Δομή Αιτήματος Πρόσκλησης Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Πρόσκλησης Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
