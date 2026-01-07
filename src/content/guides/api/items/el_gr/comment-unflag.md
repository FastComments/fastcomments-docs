[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα αφαίρεσης επισήμανσης ενός σχολίου για έναν συγκεκριμένο χρήστη.

Σημειώσεις:

- Αυτή η κλήση πρέπει πάντα να γίνεται στο πλαίσιο ενός χρήστη. Ο χρήστης μπορεί να είναι FastComments.com User, SSO User ή Tenant User.
- Αφού ένα σχόλιο αυτόματα απορριφθεί (αποκρυφτεί) - το σχόλιο μπορεί να επανεγκριθεί μόνο από διαχειριστή ή συντονιστή. Η αφαίρεση της επισήμανσης δεν θα επανεγκρίνει το σχόλιο.

[inline-code-attrs-start title = 'Αφαίρεση Επισήμανσης Σχολίου cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Για ανώνυμη επισήμανση, πρέπει να καθορίσουμε ένα `anonUserId`. Αυτό μπορεί να είναι ένα ID που αντιπροσωπεύει την ανώνυμη συνεδρία, ή ένα τυχαίο UUID.

[inline-code-attrs-start title = 'Ανώνυμη Επισήμανση Σχολίου cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης Επισήμανσης Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης Επισήμανσης Σχολίου'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
