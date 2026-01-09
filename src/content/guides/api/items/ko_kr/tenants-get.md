[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

이 API는 귀하의 테넌트가 관리하는 테넌트를 반환합니다.

페이지네이션은 `skip` 쿼리 매개변수로 제공됩니다. 테넌트는 `signUpDate` 및 `id`로 정렬되어 `100`개 단위의 페이지로 반환됩니다.

비용은 반환된 테넌트 수에 따라 결정되며, 반환된 테넌트 10개당 `1 credit per 10`가 발생합니다.

[inline-code-attrs-start title = 'Tenant cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

`Tenant` 객체에 `meta` 매개변수를 정의하고 일치하는 테넌트를 쿼리할 수 있습니다. 예를 들어, 키가 `someKey`이고 메타 값이 `some-value`인 경우, 이 키/값 쌍으로 JSON 객체를 구성한 다음 이를 쿼리 파라미터로 URI 인코딩하여 필터할 수 있습니다:

[inline-code-attrs-start title = 'Tenant Query by Meta cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 페이지네이션을 위한 건너뛸 테넌트 수. **/
    skip?: number
    /** meta 매개변수로 필터링합니다. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---