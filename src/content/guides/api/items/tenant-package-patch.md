[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to update a `TenantPackage` by `id`.

Updating a `TenantPackage` has the following restrictions:

- If you are setting `hasFlexPricing` to true, then all `flex*` parameters are required in that same request.
- The `name` may not be longer than `50 characters`.
- Each `forWhoText` item may not be longer than `200 characters`.
- Each `featureTaglines` item may not be longer than `100 characters`.
- The `TenantPackage` must be "smaller" than the parent tenant. For example, all of the `max*` parameters must have lower values than the parent tenant. 
- You may not change the `tenantId` associated with a `TenantPackage`.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Included on failure. **/
    reason?: string;
}
[inline-code-end]
