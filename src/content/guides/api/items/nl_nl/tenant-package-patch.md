[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Deze API-endpoint biedt de mogelijkheid om een `TenantPackage` op basis van `id` bij te werken.

Het bijwerken van een `TenantPackage` kent de volgende beperkingen:

- Als u `hasFlexPricing` op true zet, dan zijn alle `flex*`-parameters vereist in datzelfde verzoek.
- De `name` mag niet langer zijn dan `50 characters`.
- Elk `forWhoText`-item mag niet langer zijn dan `200 characters`.
- Elk `featureTaglines`-item mag niet langer zijn dan `100 characters`.
- De `TenantPackage` moet "kleiner" zijn dan de bovenliggende tenant. Bijvoorbeeld moeten alle `max*`-parameters lagere waarden hebben dan de bovenliggende tenant. 
- U mag de `tenantId` die aan een `TenantPackage` is gekoppeld niet wijzigen.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]