[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα αντικατάστασης ενός μόνου `TenantUser`.

Η αντικατάσταση ενός `TenantUser` έχει τους ακόλουθους περιορισμούς:

- Η `signUpDate` δεν μπορεί να είναι στο μέλλον.
- Η `locale` πρέπει να είναι στη λίστα των [Υποστηριζόμενων Τοπικών Ρυθμίσεων](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Το `username` πρέπει να είναι μοναδικό σε όλο το FastComments.com. Αν αυτό είναι πρόβλημα, προτείνουμε να χρησιμοποιήσετε SSO αντί αυτού.
- Το `email` πρέπει να είναι μοναδικό σε όλο το FastComments.com. Αν αυτό είναι πρόβλημα, προτείνουμε να χρησιμοποιήσετε SSO αντί αυτού.
- Δεν μπορείτε να ενημερώσετε το `tenantId` ενός χρήστη.

Μπορούμε να δημιουργήσουμε έναν `TenantUser` ως εξής

[inline-code-attrs-start title = 'Αντικατάσταση TenantUser cURL Παράδειγμα'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αντικατάστασης TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αντικατάστασης TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
