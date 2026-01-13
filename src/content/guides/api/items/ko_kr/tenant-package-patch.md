[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 `id`로 `TenantPackage`를 업데이트하는 기능을 제공합니다.

`TenantPackage`를 업데이트할 때 다음 제한 사항이 적용됩니다:

- `hasFlexPricing`를 true로 설정하는 경우, 같은 요청에서 모든 `flex*` 매개변수가 필요합니다.
- `name`은 `50 characters`보다 길 수 없습니다.
- 각 `forWhoText` 항목은 `200 characters`보다 길 수 없습니다.
- 각 `featureTaglines` 항목은 `100 characters`보다 길 수 없습니다.
- `TenantPackage`는 상위 테넌트보다 "작아야" 합니다. 예를 들어, 모든 `max*` 매개변수는 상위 테넌트보다 낮은 값을 가져야 합니다. 
- `TenantPackage`에 연결된 `tenantId`는 변경할 수 없습니다.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---