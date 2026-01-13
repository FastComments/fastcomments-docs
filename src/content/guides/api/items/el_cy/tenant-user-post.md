[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα προσθήκης ενός μόνου `TenantUser`.

Η δημιουργία ενός `TenantUser` έχει τους ακόλουθους περιορισμούς:

- Ένα `username` είναι απαιτούμενο.
- Ένα `email` είναι απαιτούμενο.
- Η `signUpDate` δεν μπορεί να είναι στο μέλλον.
- Η `locale` πρέπει να είναι στη λίστα των [Υποστηριζόμενων Τοπικών Ρυθμίσεων](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- Το `username` πρέπει να είναι μοναδικό σε όλο το FastComments.com. Αν αυτό είναι πρόβλημα, προτείνουμε να χρησιμοποιήσετε SSO αντί αυτού.
- Το `email` πρέπει να είναι μοναδικό σε όλο το FastComments.com. Αν αυτό είναι πρόβλημα, προτείνουμε να χρησιμοποιήσετε SSO αντί αυτού.
- Δεν μπορείτε να δημιουργήσετε περισσότερους χρήστες ενοικιαστή από αυτούς που ορίζονται στο `maxTenantUsers` στο πακέτο σας.

Μπορούμε να δημιουργήσουμε έναν `TenantUser` ως εξής

[inline-code-attrs-start title = 'Δημιουργία TenantUser cURL Παράδειγμα'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]
