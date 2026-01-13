[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint pruža mogućnost ažuriranja `TenantPackage` po `id`.

Ažuriranje `TenantPackage` ima sljedeća ograničenja:

- Ako postavite `hasFlexPricing` na true, tada su svi `flex*` parametri obavezni u tom istom zahtjevu.
- Polje `name` ne može biti duže od `50 characters`.
- Svaki element `forWhoText` ne može biti duži od `200 characters`.
- Svaki element `featureTaglines` ne može biti duži od `100 characters`.
- `TenantPackage` mora biti "manji" od roditeljskog tenanta. Na primjer, svi `max*` parametri moraju imati niže vrijednosti nego kod roditeljskog tenanta. 
- Ne smijete promijeniti `tenantId` povezan s `TenantPackage`.

[inline-code-attrs-start title = 'Primjer cURL PATCH za TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahtjeva za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

---