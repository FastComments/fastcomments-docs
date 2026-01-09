[api-resource-header-start name = 'SSOUser'; route = 'PATCH /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 SSO 사용자를 업데이트할 수 있는 기능을 제공합니다.

[inline-code-attrs-start title = 'SSOUser 업데이트 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "notfordperfect"
}'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 업데이트 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** 이메일 또는 사용자 이름을 변경할 때, 사용자의 댓글도 함께 업데이트하려면 이 값을 'true'로 설정할 수 있습니다. 이 경우 크레딧 비용이 두 배가 됩니다. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 업데이트 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-does-not-exist'
    /** 실패 시 포함됩니다. **/
    reason?: string
    user?: SSOUser; // 성공 시 업데이트된 전체 사용자를 반환합니다.
}
[inline-code-end]


---