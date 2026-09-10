[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

요청을 만든 자격 증명을 설명합니다: 해당 자격 증명이 속한 테넌트와 OAuth 액세스 토큰의 경우 애플리케이션을 승인한 사용자를 나타냅니다. 통합에서는 이를 사용하여 연결을 테스트하고 라벨을 지정합니다.

API 키를 사용할 경우 응답은 테넌트만 식별합니다. OAuth 베어러 토큰을 사용할 경우 승인된 사용자와 부여된 범위도 포함됩니다.

[inline-code-attrs-start title = 'Me cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** 요청이 인증된 방식. **/
    authType: 'api-key' | 'oauth'
    /** 자격 증명이 보유한 범위. API 키는 두 가지 모두 보유합니다. **/
    scopes: ('read' | 'write')[]
    /** OAuth 토큰에만 존재합니다. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]