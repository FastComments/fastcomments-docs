[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

이 경로는 id로 `Tenant` **및 연관된 모든 데이터** (사용자, 댓글 등)를 제거합니다.

다음과 같은 테넌트 제거에 대한 제한이 있습니다:

- 해당 테넌트는 귀하의 것이어야 하거나, 귀하가 관리하는 화이트라벨 테넌트여야 합니다.
- 쿼리 매개변수 `sure`는 `true`로 설정되어 있어야 합니다.

[inline-code-attrs-start title = '테넌트 제거 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '테넌트 삭제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '테넌트 삭제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---