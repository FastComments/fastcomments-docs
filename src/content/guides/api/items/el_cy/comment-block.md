[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα αποκλεισμού ενός χρήστη που έγραψε ένα συγκεκριμένο σχόλιο. Υποστηρίζει τον αποκλεισμό από σχόλια που γράφτηκαν από χρήστες FastComments.com, SSO Users και Tenant Users.

Υποστηρίζει μια παράμετρο body `commentIdsToCheck` για να ελέγξει αν άλλα πιθανώς ορατά σχόλια στον client θα πρέπει να αποκλειστούν/ξεμπλοκαριστούν μετά την εκτέλεση αυτής της ενέργειας.

Σημειώσεις:

- Αυτή η κλήση πρέπει πάντα να γίνεται στο πλαίσιο ενός χρήστη. Ο χρήστης μπορεί να είναι FastComments.com User, SSO User ή Tenant User.
- Το `userId` στο αίτημα είναι ο χρήστης που *κάνει τον αποκλεισμό*. Για παράδειγμα: Ο `User A` θέλει να αποκλείσει τον `User B`. Περάστε `userId=User A` και το id σχολίου που έγραψε ο `User B`.
- Εντελώς ανώνυμα σχόλια (χωρίς user id, χωρίς email) δεν μπορούν να αποκλειστούν και θα επιστραφεί σφάλμα.

[inline-code-attrs-start title = 'Αποκλεισμός Σχολίου cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Για ανώνυμο αποκλεισμό, πρέπει να καθορίσουμε ένα `anonUserId`. Αυτό μπορεί να είναι ένα ID που αντιπροσωπεύει την ανώνυμη συνεδρία, ή ένα τυχαίο UUID.
Αυτό μας επιτρέπει να υποστηρίξουμε τον αποκλεισμό σχολίων ακόμα κι αν ένας χρήστης δεν είναι συνδεδεμένος, ανακτώντας τα σχόλια με το ίδιο `anonUserId`.

[inline-code-attrs-start title = 'Ανώνυμος Αποκλεισμός Σχολίου cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αποκλεισμού Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αποκλεισμού Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
