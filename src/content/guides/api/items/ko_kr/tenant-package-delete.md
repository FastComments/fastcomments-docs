[api-resource-header-start name = 'TenantPackage'; route = 'DELETE /api/v1/tenant-packages/:id'; creditsCost = 5; api-resource-header-end]

이 경로는 id로 `TenantPackage`를 삭제합니다.

사용 중인 `TenantPackage`(테넌트의 `packageId`가 해당 패키지를 가리키는 경우)는 삭제할 수 없습니다. 먼저 `Tenant`를 업데이트하세요.

[inline-code-attrs-start title = 'TenantPackage 제거 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 제거 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 제거 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'package-in-use'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]