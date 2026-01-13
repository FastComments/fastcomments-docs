[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα δημιουργίας μιας `Subscription`. Σημειώστε ότι ένας χρήστης μπορεί να έχει μόνο μία συνδρομή ανά σελίδα, καθώς περισσότερες είναι περιττές, και η προσπάθεια
δημιουργίας περισσότερων από μία συνδρομών για τον ίδιο χρήστη για την ίδια σελίδα θα οδηγήσει σε σφάλμα.

Η δημιουργία μιας συνδρομής θα έχει ως αποτέλεσμα τη δημιουργία αντικειμένων `Notification` όταν ένα νέο σχόλιο αφήνεται στη ρίζα του συνδεδεμένου `urlId` (όταν το `parentId` του σχολίου είναι `null`).

[inline-code-attrs-start title = 'Subscription POST cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Subscription POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Subscription POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
