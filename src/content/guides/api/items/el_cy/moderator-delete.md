[api-resource-header-start name = 'Moderator'; route = 'DELETE /api/v1/moderators/:id'; creditsCost = 5; api-resource-header-end]

Αυτή η διαδρομή παρέχει την αφαίρεση ενός `Moderator` με βάση το id.

[inline-code-attrs-start title = 'Αφαίρεση Συντονιστή cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
