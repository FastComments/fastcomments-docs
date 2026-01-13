[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава ажурирање `TenantPackage` по `id`.

Ажурирање `TenantPackage` има следећа ограничења:

- Ако постављате `hasFlexPricing` на true, онда су сви `flex*` параметри обавезни у истом захтеву.
- `name` не сме бити дужи од `50 characters`.
- Сваки `forWhoText` ставка не сме бити дужа од `200 characters`.
- Сваки `featureTaglines` ставка не сме бити дужи од `100 characters`.
- `TenantPackage` мора бити "smaller" од родитељског тенанта. На пример, сви `max*` параметри морају имати ниже вредности од родитељског тенанта. 
- Не можете променити `tenantId` повезан са `TenantPackage`.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---