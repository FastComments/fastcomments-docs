[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα αποστολής συνδέσμου σύνδεσης σε έναν μόνο `TenantUser`.

Χρήσιμο όταν δημιουργείτε χρήστες μαζικά και δεν θέλετε να τους καθοδηγήσετε για τον τρόπο σύνδεσης στο FastComments.com. Αυτό απλά θα τους στείλει έναν "μαγικό σύνδεσμο" για σύνδεση που
λήγει μετά από `30 ημέρες`.

Οι ακόλουθοι περιορισμοί υπάρχουν για την αποστολή συνδέσμου σύνδεσης σε έναν `TenantUser`:
- Ο `TenantUser` πρέπει να υπάρχει ήδη.
- Πρέπει να έχετε πρόσβαση στη διαχείριση του `Tenant` στον οποίο ανήκει ο `TenantUser`.

Μπορούμε να στείλουμε σύνδεσμο σύνδεσης σε έναν `TenantUser` ως εξής:

[inline-code-attrs-start title = 'Σύνδεσμος Σύνδεσης TenantUser cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Αυτό θα στείλει ένα email όπως `Ο Bob στο TenantName σας προσκαλεί να γίνετε συντονιστής...`

[inline-code-attrs-start title = 'Δομή Αιτήματος Συνδέσμου Σύνδεσης TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Συνδέσμου Σύνδεσης TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
