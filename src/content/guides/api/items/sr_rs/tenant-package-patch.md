[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Ова API крајња тачка пружа могућност ажурирања `TenantPackage` по `id`.

Ажурирање `TenantPackage` има следећа ограничења:

- Ако постављате `hasFlexPricing` на true, онда су сви `flex*` параметри потребни у истом захтеву.
- `name` не сме бити дужи од `50 characters`.
- Свака ставка `forWhoText` не сме бити дужа од `200 characters`.
- Свака ставка `featureTaglines` не сме бити дужа од `100 characters`.
- `TenantPackage` мора бити „мањи“ од родитељског tenant-а. На пример, сви `max*` параметри морају имати нижe вредности од родитељског tenant-а. 
- Не можете променити `tenantId` повезан са `TenantPackage`.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Укључено при неуспеху. **/
    reason?: string
}
[inline-code-end]

---