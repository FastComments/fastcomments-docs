[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

이 API는 `skip`, `limit`, `before`, `after` 매개변수를 사용한 페이지네이션을 제공합니다. AuditLogs는 기본적으로 `100`개의 페이지로 반환되며, 최대 `limit`은 `200`이며, `when` 및 `id` 순으로 정렬됩니다.

반환되는 매 `100`개의 로그마다 크레딧 비용은 `1`입니다.

기본적으로, **가장 최신 항목이 먼저**인 목록을 받게 됩니다. 이렇게 하면 `skip=0`부터 시작하여 폴링하고, 소비한 마지막 레코드를 찾을 때까지 페이지네이션할 수 있습니다.

또는 오래된 순으로 정렬하고, 더 이상 레코드가 없을 때까지 페이지네이션할 수 있습니다.

`order`를 `ASC` 또는 `DESC`로 설정하여 정렬할 수 있습니다. 기본값은 `DESC`입니다.

`before`와 `after`를 밀리초 단위 타임스탬프로 사용하여 날짜별 조회가 가능합니다. `before`와 `after`는 포함되지 않으며, 각각 단독으로 사용할 수 있습니다.

## 사람에게 무슨 일이 있었는지 찾기

각 이벤트는 누가 수행했는지(`username`, `userId`, `ip`)와 수행된 대상을 별도로 기록합니다. `targetLabel`은 해당 객체에 대한 사람이 읽을 수 있는 레이블이며, 예를 들어 `jsmith (jsmith@example.com)`와 같습니다. `targetId`는 해당 객체의 ID입니다. 사람의 이름이나 이메일은 알지만 ID를 모를 경우 레이블에 대한 대소문자 구분 없는 부분 문자열 매치를 위해 `target`을 사용하세요.

삭제 이벤트는 발생 시점의 레이블을 캡처하므로, 기본 레코드가 사라진 후에도 삭제된 사용자나 모더레이터를 식별할 수 있습니다.

## 관리되는 테넌트

테넌트가 다른 테넌트를 관리하는 경우, `includeManagedTenants=true`로 설정하면 해당 테넌트와 관리하는 모든 테넌트의 이벤트를 하나의 응답으로 반환합니다. 반환된 각 로그의 `tenantId`는 해당 로그가 어느 테넌트에서 왔는지 알려줍니다.

[inline-code-attrs-start title = 'AuditLog cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** 이 사용자 이름으로 수행된 이벤트만. **/
    username?: string
    /** 이 IP 주소에서 발생한 이벤트만. **/
    ip?: string
    /** 이 유형의 이벤트만. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** 이 리소스에 대한 이벤트만, 예: 사용자 또는 모더레이터. **/
    resourceName?: string
    /** 영향을 받은 객체가 이 ID를 가진 이벤트만. **/
    targetId?: string
    /** 영향을 받은 객체 레이블에 대한 대소문자 구분 없는 부분 문자열 매치. **/
    target?: string
    /** 이 테넌트가 관리하는 테넌트의 이벤트도 반환합니다. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됨. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** 실패 시 포함됨. **/
    reason?: string
    /** 로그! **/
    auditLogs: AuditLog[]
}
[inline-code-end]