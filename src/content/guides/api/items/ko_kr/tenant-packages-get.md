[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

이 API는 페이징을 사용하며, `skip` 쿼리 매개변수로 제공됩니다. TenantPackages는 `100`개 단위로 페이지별로 반환되며, `createdAt` 및 `id`로 정렬됩니다.

비용은 반환되는 tenant packages 수를 기준으로 하며, 반환되는 tenant packages 10개당 `1 credit per 10`가 소요됩니다.

[inline-code-attrs-start title = 'TenantPackage cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 페이지네이션을 위해 건너뛸 tenant packages의 수. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---