[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, ein `TenantPackage` nach `id` zu aktualisieren.

Das Aktualisieren eines `TenantPackage` hat die folgenden Einschränkungen:

- Wenn Sie `hasFlexPricing` auf true setzen, sind alle `flex*`-Parameter in derselben Anfrage erforderlich.
- Der `name` darf nicht länger als `50 Zeichen` sein.
- Jedes `forWhoText`-Element darf nicht länger als `200 Zeichen` sein.
- Jedes `featureTaglines`-Element darf nicht länger als `100 Zeichen` sein.
- Das `TenantPackage` muss "kleiner" als der übergeordnete Tenant sein. Beispielsweise müssen alle `max*`-Parameter niedrigere Werte als der übergeordnete Tenant haben.
- Sie können die mit einem `TenantPackage` verknüpfte `tenantId` nicht ändern.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
