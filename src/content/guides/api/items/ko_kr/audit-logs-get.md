[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

이 API는 `skip`, `before`, `after` 매개변수로 제공되는 페이지네이션을 사용합니다. AuditLogs는 `when`과 `id`로 정렬되며 한 페이지당 `1000`개가 반환됩니다.

매 `1000`개의 로그를 가져오는 데는 크레딧 `10`이 소모됩니다.

기본적으로, **가장 최신 항목이 먼저**인 목록을 받습니다. 이렇게 하면 `skip=0`부터 폴링을 시작하여 마지막으로 처리한 레코드를 찾을 때까지 페이지네이션할 수 있습니다.

또는 오래된 항목부터 정렬하여 더 이상 레코드가 없을 때까지 페이지네이션할 수 있습니다.

정렬은 `order`를 `ASC` 또는 `DESC`로 설정하여 수행할 수 있습니다. 기본값은 `ASC`입니다.

날짜로 쿼리하는 것은 밀리초 단위 타임스탬프로 `before`와 `after`를 사용하는 방식으로 가능합니다. `before`와 `after`는 포함되지 않습니다.

[inline-code-attrs-start title = 'AuditLog cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 로그들! **/
    auditLogs: AuditLog[]
}
[inline-code-end]