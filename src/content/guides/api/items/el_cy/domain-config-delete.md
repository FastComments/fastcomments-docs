[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει την αφαίρεση ενός μόνο `DomainConfig` με βάση το id.

- Σημείωση: Η αφαίρεση ενός `DomainConfig` θα αφαιρέσει την εξουσιοδότηση αυτού του domain από τη χρήση του FastComments.
- Σημείωση: Η επαναπροσθήκη ενός domain μέσω του UI θα αναδημιουργήσει το αντικείμενο (με μόνο το `domain` συμπληρωμένο).

[inline-code-attrs-start title = 'Αφαίρεση DomainConfig cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
