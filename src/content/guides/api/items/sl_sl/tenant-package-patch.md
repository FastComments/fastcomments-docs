[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča posodobitev `TenantPackage` po `id`.

Posodabljanje `TenantPackage` ima naslednje omejitve:

- Če nastavite `hasFlexPricing` na true, so vsi parametri `flex*` obvezni v istem zahtevku.
- `name` ne sme biti daljše od `50 characters`.
- Vsak element `forWhoText` ne sme biti daljši od `200 characters`.
- Vsak element `featureTaglines` ne sme biti daljši od `100 characters`.
- `TenantPackage` mora biti "manjši" od nadrejenega najemnika. Na primer, vsi `max*` parametri morajo imeti nižje vrednosti kot pri nadrejenem najemniku. 
- Ne smete spremeniti `tenantId`, povezanega z `TenantPackage`.

[inline-code-attrs-start title = 'Primer cURL PATCH za TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahtevka za TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]

---