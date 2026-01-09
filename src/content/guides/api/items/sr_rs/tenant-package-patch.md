[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje `TenantPackage` pomoću `id`.

Ažuriranje `TenantPackage` ima sledeća ograničenja:

- Ako postavite `hasFlexPricing` na true, onda su svi `flex*` parametri obavezni u istom zahtevu.
- `name` ne može biti duži od `50 characters`.
- Svaka stavka `forWhoText` ne može biti duža od `200 characters`.
- Svaka stavka `featureTaglines` ne može biti duža od `100 characters`.
- `TenantPackage` mora biti "manji" od nadređenog tenanta. Na primer, svi `max*` parametri moraju imati niže vrednosti od nadređenog tenanta. 
- Ne možete promeniti `tenantId` povezan sa `TenantPackage`.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahteva za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]

---