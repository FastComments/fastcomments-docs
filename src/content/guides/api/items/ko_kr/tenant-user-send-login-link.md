[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

이 경로는 단일 `TenantUser`에게 로그인 링크를 보낼 수 있는 기능을 제공합니다.

사용자들을 일괄 생성할 때 그들에게 FastComments.com에 로그인하는 방법을 안내할 필요가 없을 때 유용합니다. 이는 만료 기간이 `30 days`인 "매직 링크"를 전송합니다.

`TenantUser`에게 로그인 링크를 보내기 위해 다음과 같은 제한이 있습니다:
- `TenantUser`는 이미 존재해야 합니다.
- `TenantUser`가 속한 `Tenant`를 관리할 수 있는 권한이 있어야 합니다.

다음과 같이 `TenantUser`에게 로그인 링크를 보낼 수 있습니다:

[inline-code-attrs-start title = 'TenantUser 로그인 링크 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

이것은 `Bob at TenantName is inviting you to be a moderator...` 같은 이메일을 보냅니다.

[inline-code-attrs-start title = 'TenantUser 로그인 링크 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 로그인 링크 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]