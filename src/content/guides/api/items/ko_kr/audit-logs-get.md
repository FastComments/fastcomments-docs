[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

이 API는 `skip`, `limit`, `before`, `after` 파라미터로 제공되는 페이지네이션을 사용합니다. AuditLog는 기본적으로 `1000`개씩 페이지로 반환되며, 최대 `limit`은 `10000`이고, `when`과 `id` 순으로 정렬됩니다. 이 엔드포인트는 대화형으로 페이지를 넘기는 용도보다는 주로 히스토리를 한 번에 덤프하는 용도로 사용되기 때문에 페이지 크기가 큽니다.

반환되는 로그 `100`개마다 크레딧 비용은 `1`입니다.

기본적으로 **가장 최신 항목부터** 목록을 받게 됩니다. 이렇게 하면 `skip=0`부터 폴링을 시작하여 마지막으로 소비한 레코드를 찾을 때까지 페이지네이션할 수 있습니다.

또는 가장 오래된 항목부터 정렬하여 더 이상 레코드가 없을 때까지 페이지네이션할 수도 있습니다.

정렬은 `order`를 `ASC` 또는 `DESC`로 설정하여 수행할 수 있습니다. 기본값은 `DESC`입니다.

날짜 기준 조회는 밀리초 단위 타임스탬프인 `before`와 `after`를 통해 가능합니다. `before`와 `after`는 포함되지 않으며(inclusive하지 않음), 둘 중 하나만 단독으로 사용할 수도 있습니다.

## 특정 사람에게 발생한 일 찾기

모든 이벤트는 수행한 사람(`username`, `userId`, `ip`)과, 별도로 수행 대상이 된 객체를 기록합니다. `targetLabel`은 해당 객체의 사람이 읽을 수 있는 라벨(예: `jsmith (jsmith@example.com)`)이고, `targetId`는 그 id입니다. 사람의 이름이나 이메일은 알지만 id는 모를 때, 라벨에 대한 대소문자 구분 없는 부분 문자열 매칭을 위해 `target`을 사용하세요.

삭제 이벤트는 이벤트 시점의 라벨을 캡처하므로, 원본 레코드가 사라진 후에도 삭제된 사용자나 모더레이터를 식별할 수 있습니다.

## 관리되는 테넌트

귀하의 테넌트가 다른 테넌트를 관리하는 경우, `includeManagedTenants=true`로 설정하면 귀하의 테넌트와 관리하는 모든 테넌트의 이벤트를 하나의 응답으로 받을 수 있습니다. 반환된 각 로그의 `tenantId`를 통해 어떤 테넌트에서 온 것인지 알 수 있습니다.

[inline-code-attrs-start title = 'AuditLog cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** 최대 10000. 기본값은 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** 이 username이 수행한 이벤트만. **/
    username?: string
    /** 이 IP 주소의 이벤트만. **/
    ip?: string
    /** 이 유형의 이벤트만. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** 이 리소스(예: User 또는 Moderator)의 이벤트만. **/
    resourceName?: string
    /** 영향을 받은 객체의 id가 이 값인 이벤트만. **/
    targetId?: string
    /** 영향을 받은 객체의 라벨에 대한 대소문자 구분 없는 부분 문자열 매칭. **/
    target?: string
    /** 이 테넌트가 관리하는 테넌트의 이벤트도 반환. **/
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
    /** 로그 목록! **/
    auditLogs: AuditLog[]
}
[inline-code-end]