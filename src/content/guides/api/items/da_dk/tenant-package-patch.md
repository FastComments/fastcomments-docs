[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Dette API-endpoint giver mulighed for at opdatere en `TenantPackage` efter `id`.

Opdatering af en `TenantPackage` har følgende begrænsninger:

- Hvis du sætter `hasFlexPricing` til true, så er alle `flex*`-parametre påkrævet i den samme anmodning.
- `name` må ikke være længere end `50 tegn`.
- Hvert `forWhoText`-element må ikke være længere end `200 tegn`.
- Hvert `featureTaglines`-element må ikke være længere end `100 tegn`.
- `TenantPackage` skal være "mindre" end den overordnede tenant. For eksempel skal alle `max*`-parametre have lavere værdier end den overordnede tenant.
- Du kan ikke ændre `tenantId` tilknyttet en `TenantPackage`.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
