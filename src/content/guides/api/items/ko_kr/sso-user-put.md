[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 SSO 사용자를 업데이트할 수 있는 기능을 제공합니다.

[inline-code-attrs-start title = 'SSOUser 업데이트 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

이 예에서는 접근 제어를 위해 `groupIds`를 지정하지만, 이는 선택 사항입니다.

[inline-code-attrs-start title = 'SSOUser 업데이트 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 이메일 또는 사용자 이름이 변경될 때, 이 값을 true로 설정하면 사용자의 댓글도 함께 업데이트합니다. 이 경우 크레딧 비용은 두 배가 됩니다. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 업데이트 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** 실패 시 포함됩니다. **/
    reason?: string
    user?: SSOUser; // 성공 시 업데이트된 사용자를 반환합니다.
}
[inline-code-end]