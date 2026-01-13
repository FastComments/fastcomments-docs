[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει την αφαίρεση ενός μόνο χρήστη SSO με βάση το id του.

Σημειώστε ότι η φόρτωση του widget σχολίων ξανά με ένα φορτίο για αυτόν τον χρήστη θα αναδημιουργήσει απλά τον χρήστη απρόσκοπτα.

Η διαγραφή των σχολίων του χρήστη είναι δυνατή μέσω της παραμέτρου ερωτήματος `deleteComments`. Σημειώστε ότι αν αυτή είναι αληθής:

1. Όλα τα σχόλια του χρήστη θα διαγραφούν ζωντανά.
2. Όλα τα __παιδικά__ (τώρα ορφανά) σχόλια θα διαγραφούν ή θα ανωνυμοποιηθούν με βάση τη διαμόρφωση της συσχετισμένης σελίδας κάθε σχολίου. Για παράδειγμα αν η λειτουργία διαγραφής νήματος είναι "ανωνυμοποίηση", τότε οι απαντήσεις θα παραμείνουν, και τα σχόλια του χρήστη θα ανωνυμοποιηθούν. Αυτό ισχύει μόνο όταν το `commentDeleteMode` είναι `Remove` (η προεπιλεγμένη τιμή).
3. Το `creditsCost` γίνεται `2`.

### Ανωνυμοποιημένα Σχόλια

Μπορείτε να διατηρήσετε τα σχόλια του χρήστη αλλά απλά να τα ανωνυμοποιήσετε ορίζοντας `commentDeleteMode=1`.

Αν τα σχόλια του χρήστη ανωνυμοποιηθούν τότε οι ακόλουθες τιμές ορίζονται σε null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

Τα `isDeleted` και `isDeletedUser` ορίζονται σε `true`.

Κατά την απόδοση, το widget σχολίων θα χρησιμοποιεί το `DELETED_USER_PLACEHOLDER` (προεπιλογή: "[deleted]") για το όνομα του χρήστη και το `DELETED_CONTENT_PLACEHOLDER` για το σχόλιο. Αυτά μπορούν να προσαρμοστούν μέσω του UI Προσαρμογής Widget.

### Παραδείγματα

[inline-code-attrs-start title = 'Αφαίρεση SSOUser cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the removed user on success.
}
[inline-code-end]
