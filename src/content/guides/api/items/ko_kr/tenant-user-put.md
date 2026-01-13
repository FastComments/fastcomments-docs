[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 `TenantUser`를 교체할 수 있는 기능을 제공합니다.

`TenantUser`를 교체할 때 다음과 같은 제한이 있습니다:

- `signUpDate`는 미래 날짜일 수 없습니다.
- `locale`은 [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 목록에 있어야 합니다.
- `username`은 FastComments.com 전체에서 고유해야 합니다. 문제가 될 경우, 대신 SSO 사용을 권장합니다.
- `email`은 FastComments.com 전체에서 고유해야 합니다. 문제가 될 경우, 대신 SSO 사용을 권장합니다.
- 사용자의 `tenantId`는 업데이트할 수 없습니다.

다음과 같이 `TenantUser`를 교체할 수 있습니다

[inline-code-attrs-start title = 'TenantUser 교체 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 교체 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 이메일 또는 username이 변경될 때 이 값을 true로 설정하면 사용자의 댓글도 함께 업데이트됩니다. 이 경우 크레딧 비용이 두 배가 됩니다. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 교체 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]