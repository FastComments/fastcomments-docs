[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 SSO 사용자를 생성합니다.

동일한 ID로 두 사용자를 생성하려 하면 오류가 발생합니다.

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

In this example we specify `groupIds` for access control, but this is optional.

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
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** 실패 시 포함됩니다. **/
    reason?: string
    user?: SSOUser; // 성공 시 생성된 사용자를 반환합니다.
}
[inline-code-end]

#### 통합 참고

API로 전달된 데이터는 다른 SSO User HMAC 페이로드를 전달하는 것만으로 간단히 덮어쓸 수 있습니다. For example, if
API를 통해 username을 설정했지만 페이지 로드 시 SSO 흐름에서 다른 username을 전달하면, 우리는 자동으로 업데이트
해당 사용자의 username.

We will not update user parameters in this flow unless you explicitly specify them or set them to null (not undefined).