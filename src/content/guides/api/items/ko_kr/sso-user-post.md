[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 SSO 사용자의 생성을 제공합니다.

동일한 ID로 두 명의 사용자를 생성하려고 하면 오류가 발생합니다.

[inline-code-attrs-start title = 'SSOUser 생성 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

이 예제에서는 접근 제어를 위해 `groupIds`를 지정하지만, 이는 선택 사항입니다.

[inline-code-attrs-start title = 'SSOUser 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### 통합 참고 사항

API를 통해 전달된 데이터는 다른 SSO 사용자 HMAC 페이로드를 전달하는 것만으로 간단히 재정의할 수 있습니다. 예를 들어,
API를 통해 사용자 이름을 설정한 후, 페이지 로드 시 SSO 흐름을 통해 다른 사용자 이름을 전달하면 자동으로
해당 사용자의 사용자 이름이 업데이트됩니다.

이 흐름에서는 사용자 매개변수를 명시적으로 지정하거나 null로 설정하지 않는 한( undefined가 아닌 경우) 업데이트하지 않습니다.