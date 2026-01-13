[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

이 API는 페이징을 사용하며, `skip` 쿼리 매개변수로 제공됩니다. 모더레이터는 `createdAt` 및 `id` 순으로 정렬되어 한 페이지에 `100`명씩 반환됩니다.

요금은 반환된 모더레이터 수에 따라 계산되며, 반환된 모더레이터 10명당 `1 credit per 10`의 비용이 발생합니다.

[inline-code-attrs-start title = '모더레이터 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '모더레이터 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 페이징을 위해 건너뛸 모더레이터 수. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = '모더레이터 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]