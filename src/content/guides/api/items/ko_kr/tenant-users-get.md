[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

이 API는 페이지네이션을 사용하며, `skip` 쿼리 매개변수로 제공됩니다. TenantUsers는 `100`개 단위의 페이지로 반환되며 `signUpDate`, `username` 및 `id` 순으로 정렬됩니다.

비용은 반환되는 tenant users 수를 기준으로 계산되며, 반환되는 tenant users 10명당 `1 credit`이 소모됩니다.

[inline-code-attrs-start title = 'TenantUser cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 페이지네이션을 위해 건너뛸 tenant users 수. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]