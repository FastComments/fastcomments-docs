FastComments는 OAuth 2.1 인증 서버입니다. 애플리케이션은 FastComments 계정 하나에 연결된 토큰을 얻어 이 가이드의 모든 엔드포인트에서 API 키 대신 사용할 수 있습니다. 이는 Zapier 앱, MCP 서버 및 기타 서드파티 통합이 연결되는 방식입니다.

토큰은 PKCE를 사용한 인증 코드 흐름을 통해 발급됩니다. 클라이언트 자격 증명 또는 암시적 부여는 없습니다.

### Discovery

엔드포인트 위치, 지원되는 부여 유형 및 인증 방법은 표준 메타데이터 URL에 게시됩니다:

[inline-code-attrs-start title = '인증 서버 메타데이터'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

그가 설명하는 엔드포인트:

[inline-code-attrs-start title = 'OAuth 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU 지역 계정은 발급자로 `https://eu.fastcomments.com`을 사용하며, 경로는 동일합니다.

### Registering a client

클라이언트는 흐름을 시작하기 전에 `client_id`와 등록된 `redirect_uri`가 필요합니다. 이를 얻는 방법은 두 가지가 있습니다:

- **동적 클라이언트 등록.** RFC 7591에 따라 JSON 본문(`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`)을 포함한 `POST /oauth/register`. 응답에는 `client_id`와 기밀 클라이언트의 경우 `client_secret`이 포함됩니다. 등록은 인증 없이 이루어지며 IP당 속도 제한이 적용됩니다.
- **클라이언트 ID 메타데이터 문서.** 클라이언트는 자신이 제어하는 `https` URL을 `client_id`로 사용합니다. FastComments는 해당 URL을 가져와 동일한 메타데이터 필드를 읽습니다. 별도의 등록 호출이 필요하지 않습니다.

FastComments 대시보드에 나열된 파트너 애플리케이션(예: Zapier)은 FastComments가 직접 등록합니다. 마켓플레이스 목록을 구축하고 1인당 클라이언트가 필요한 경우 지원팀에 문의하십시오.

### Scopes

[inline-code-attrs-start title = '스코프'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

스코프를 지정하지 않은 요청은 두 스코프 모두 부여됩니다. 사용자는 동의 페이지에서 요청된 스코프를 확인합니다. 이 두 스코프 외의 스코프를 요청하면 `invalid_scope` 오류가 발생합니다.

### Step 1 - Authorization request

사용자의 브라우저를 인증 엔드포인트로 보냅니다. 모든 클라이언트는 `S256` 방법을 사용한 PKCE가 필요합니다.

[inline-code-attrs-start title = '인증 요청'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = '인증 요청 매개변수'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** 클라이언트가 등록한 리디렉션 URI 중 하나와 정확히 일치해야 합니다. **/
    redirect_uri: string
    /** 공백으로 구분됩니다. 두 스코프를 모두 요청하려면 생략합니다. **/
    scope?: 'read' | 'write' | 'read write'
    /** 리디렉션 시 변경되지 않은 상태로 반환됩니다. 흐름을 시작한 세션에 콜백을 연결하는 데 사용합니다. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** 선택적 RFC 8707 리소스 표시자입니다. 전송할 경우 토큰 엔드포인트에도 동일한 값을 전송해야 합니다. **/
    resource?: string
}
[inline-code-end]

필요한 경우 사용자는 FastComments에 로그인하고 애플리케이션 이름, 연결될 계정 및 요청된 스코프가 표시된 동의 페이지를 봅니다. 사용자는 해당 계정에 대해 **API Admin** 권한을 보유해야 하며, 다른 사용자는 동의 양식 대신 권한 오류를 보게 됩니다. 승인을 하면 브라우저가 `code`와 `state`를 포함한 `redirect_uri`로 리디렉션됩니다. 거부하면 `error=access_denied`와 함께 리디렉션됩니다.

인증 코드는 10분 동안 유효하며 한 번만 교환할 수 있습니다. 동일한 코드를 두 번째로 교환하면 첫 번째 교환으로 생성된 모든 토큰이 폐기됩니다.

### Step 2 - Token request

코드를 토큰으로 교환합니다. 본문은 폼 인코딩됩니다. 기밀 클라이언트는 `client_secret_basic`(HTTP Basic) 또는 `client_secret_post`(본문에 비밀)으로 인증합니다. 공개 클라이언트는 `client_id`만 전송합니다.

[inline-code-attrs-start title = '토큰 요청 cURL 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = '토큰 요청 본문 (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** 기밀 클라이언트 전용. 대신 HTTP Basic 인증으로 보낼 수 있습니다. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = '토큰 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** 접두사 fcat_ . 유효 기간은 1시간입니다. **/
    access_token: string
    token_type: 'bearer'
    /** 액세스 토큰이 만료될 때까지 남은 초. 3600. **/
    expires_in: number
    /** 접두사 fcrt_ . 발급 후 30일 동안 유효합니다. **/
    refresh_token: string
    /** 공백으로 구분된 부여된 스코프. **/
    scope: string
}
[inline-code-end]

오류는 RFC 6749를 따릅니다: `error`와 `error_description`을 포함한 JSON 본문, `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target`, `unsupported_grant_type`에 대해 HTTP 400, `invalid_client`에 대해 HTTP 401, 속도 제한 시 HTTP 429.

### Step 3 - Calling the API

액세스 토큰을 Bearer 토큰으로 전송합니다. 테넌트는 토큰에 내포되므로 `tenantId`는 선택 사항입니다. 제공할 경우 토큰과 일치해야 하며, 일치하지 않으면 요청이 실패합니다.

[inline-code-attrs-start title = 'Bearer 토큰 cURL 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me`는 테넌트, 인증된 사용자 및 부여된 스코프를 반환하므로 연결 테스트에 적합한 호출입니다. 만료되었거나 폐기된 토큰으로 요청하면 HTTP 401이 반환됩니다. 토큰이 보유하지 않은 스코프가 필요한 메서드로 요청하면 HTTP 403이 반환됩니다.

### Step 4 - Refreshing

[inline-code-attrs-start title = '리프레시 요청 cURL 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = '토큰 요청 본문 (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** 선택 사항. 원래 부여된 스코프의 하위 집합으로 제한합니다. **/
    scope?: string
    resource?: string
}
[inline-code-end]

응답은 코드 교환과 동일한 구조를 가집니다. 리프레시 토큰은 회전합니다: 각 리프레시 시 새로운 `refresh_token`을 반환하고, 동시 요청을 위한 30초의 유예 기간 후에 이전 토큰을 폐기합니다. 30초 이상 지난 회전된 리프레시 토큰을 제시하면 재생으로 간주되어 전체 부여가 폐기됩니다. FastComments가 등록한 파트너 애플리케이션은 회전에서 제외되며, 동일한 리프레시 토큰을 반환하고 만료 기간이 추가로 30일 연장됩니다.

리프레시 시 인증된 사용자가 여전히 계정에 API Admin 권한을 보유하고 있는지도 재검사합니다. 권한이 없으면 부여가 폐기되고 응답은 `invalid_grant`가 됩니다.

### Revocation

[inline-code-attrs-start title = '폐기 요청 cURL 예시'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

리프레시 토큰을 폐기하면 동일한 부여에서 발급된 모든 액세스 토큰이 폐기됩니다. 액세스 토큰을 폐기하면 해당 토큰만 폐기됩니다. 엔드포인트는 RFC 7009에 따라 토큰 존재 여부와 관계없이 빈 JSON 객체와 함께 HTTP 200을 반환합니다.

사용자는 FastComments 대시보드의 **Connected Apps**에서 연결을 폐기할 수도 있습니다. 해당 애플리케이션의 모든 토큰이 즉시 작동을 멈춥니다.