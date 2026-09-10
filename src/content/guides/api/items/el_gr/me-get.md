[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Περιγράφει τα διαπιστευτήρια που κάνουν το αίτημα: τον ενοικιαστή στον οποίο ανήκουν και, για διακριτικά πρόσβασης OAuth, τον χρήστη που εξουσιοδότησε την εφαρμογή. Οι ενσωματώσεις το χρησιμοποιούν για να δοκιμάσουν μια σύνδεση και να την επισημάνουν.

Με ένα κλειδί API η απόκριση αναγνωρίζει μόνο τον ενοικιαστή. Με ένα διακριτικό φορέα OAuth μεταφέρει επίσης τον εξουσιοδοτημένο χρήστη και τα πεδία (scopes) που χορηγήθηκαν.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** How the request was authenticated. **/
    authType: 'api-key' | 'oauth'
    /** The scopes the credential holds. API keys hold both. **/
    scopes: ('read' | 'write')[]
    /** Only present for OAuth tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]