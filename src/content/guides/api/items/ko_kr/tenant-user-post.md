[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `TenantUser`를 추가하는 기능을 제공합니다.

`TenantUser`를 생성할 때 다음과 같은 제한이 있습니다:

- `username`은 필수입니다.
- `email`은 필수입니다.
- `signUpDate`는 미래일 수 없습니다.
- `locale`은 [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 목록에 있어야 합니다.
- `username`은 FastComments.com 전체에서 고유해야 합니다. 문제가 발생하면 대신 SSO 사용을 권장합니다.
- `email`은 FastComments.com 전체에서 고유해야 합니다. 문제가 발생하면 대신 SSO 사용을 권장합니다.
- 패키지에 정의된 `maxTenantUsers`보다 더 많은 테넌트 사용자를 생성할 수 없습니다. 

다음과 같이 `TenantUser`를 생성할 수 있습니다

[inline-code-attrs-start title = 'TenantUser 생성 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenantUser?: TenantUser; // 성공 시 생성된 전체 테넌트 사용자를 반환합니다.
}
[inline-code-end]

---