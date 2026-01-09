[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje `TenantPackage` po `id`.

Ažuriranje `TenantPackage` ima sledeća ograničenja:

- Ako postavljate `hasFlexPricing` na `true`, onda su svi `flex*` parametri obavezni u tom istom zahtevu.
- `name` ne sme biti duže od `50 characters`.
- Svaki element `forWhoText` ne sme biti duži od `200 characters`.
- Svaki element `featureTaglines` ne sme biti duži od `100 characters`.
- `TenantPackage` mora biti "manji" od roditeljskog tenanta. Na primer, svi `max*` parametri moraju imati niže vrednosti od roditeljskog tenanta. 
- Ne smete menjati `tenantId` povezan sa `TenantPackage`.

[inline-code-attrs-start title = 'Primer cURL zahteva za TenantPackage PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za TenantPackage PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za TenantPackage PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]