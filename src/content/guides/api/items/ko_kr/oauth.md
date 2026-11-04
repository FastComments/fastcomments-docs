FastComments는 OAuth 2.1 인가 서버입니다. 애플리케이션은 하나의 FastComments 계정에 바인딩된 토큰을 발급받아 이 가이드의 모든 엔드포인트에서 API 키 대신 사용할 수 있습니다. Zapier 앱, MCP 서버 및 기타 서드파티 통합이 이 방식으로 연결됩니다.

토큰은 PKCE가 적용된 인가 코드 플로우를 통해 발급됩니다. 클라이언트 자격 증명(client credentials) 또는 암시적(implicit) 그랜트는 지원하지 않습니다.

### 디스커버리

엔드포인트 위치, 지원되는 그랜트, 인증 방식은 표준 메타데이터 URL에서 확인할 수 있습니다:

[inline-code-attrs-start title = '인가 서버 메타데이터'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

해당 메타데이터가 설명하는 엔드포인트:

[inline-code-attrs-start title = 'OAuth 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU 리전의 계정은 동일한 경로로 `https://eu.fastcomments.com`를 issuer로 사용합니다.

### 클라이언트 등록

클라이언트는 플로우를 시작하기 전에 `client_id`와 등록된 `redirect_uri`가 필요합니다. 두 가지 방법이 있습니다:

- **동적 클라이언트 등록(Dynamic Client Registration).** RFC 7591에 따라 JSON 본문(`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`)과 함께 `POST /oauth/register`를 호출합니다. 응답에는 `client_id`가 포함되며, 기밀(confidential) 클라이언트의 경우 `client_secret`도 포함됩니다. 등록은 인증 없이 수행되며 IP별로 요청이 제한됩니다.
- **클라이언트 ID 메타데이터 문서(Client ID Metadata Document).** 클라이언트가 자신이 제어하는 `https` URL을 `client_id`로 사용합니다. FastComments가 해당 URL을 가져와 동일한 메타데이터 필드를 읽습니다. 별도의 등록 호출이 필요하지 않습니다.

Zapier 등 FastComments 대시보드에 나열된 파트너 애플리케이션은 FastComments가 직접 등록합니다. 마켓플레이스 등록 항목을 만들고 자사(first-party) 클라이언트가 필요한 경우 지원팀에 문의하세요.

### 스코프

[inline-code-attrs-start title = '스코프'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

스코프를 지정하지 않은 요청은 두 스코프가 모두 부여됩니다. 사용자는 동의 페이지에서 요청된 스코프를 확인합니다. 이 두 가지 이외의 스코프를 요청하면 `invalid_scope` 오류로 실패합니다.

### 1단계 - 인가 요청

사용자의 브라우저를 authorize 엔드포인트로 보냅니다. `S256` 방식의 PKCE는 모든 클라이언트에 필수입니다.

[inline-code-attrs-start title = '인가 요청'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '인가 요청 파라미터'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** 클라이언트에 등록된 리다이렉트 URI 중 하나와 정확히 일치해야 합니다. **/
    redirect_uri: string
    /** 공백으로 구분. 생략하면 두 스코프가 모두 요청됩니다. **/
    scope?: 'read' | 'write' | 'read write'
    /** 리다이렉트 시 변경 없이 반환됩니다. 콜백을 플로우를 시작한 세션에 바인딩하는 데 사용하세요. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** 선택적 RFC 8707 리소스 인디케이터. 전송한 경우 동일한 값을 토큰 엔드포인트에도 전송해야 합니다. **/
    resource?: string
}
[inline-code-end]

필요한 경우 사용자가 FastComments에 로그인한 뒤, 애플리케이션 이름, 연결될 계정, 요청된 스코프가 표시된 동의 페이지를 보게 됩니다. 사용자는 해당 계정에서 **API Admin** 권한을 보유해야 하며, 그렇지 않은 사용자는 동의 폼 대신 권한 오류를 보게 됩니다. 승인하면 `code`와 `state`와 함께 브라우저가 `redirect_uri`로 리다이렉트됩니다. 거부하면 `error=access_denied`와 함께 리다이렉트됩니다.

인가 코드는 10분간 유효하며 한 번만 교환할 수 있습니다. 동일한 코드를 두 번째로 교환하면 첫 번째 교환에서 발급된 모든 토큰이 취소됩니다.

### 2단계 - 토큰 요청

코드를 토큰으로 교환합니다. 본문은 form 인코딩입니다. 기밀 클라이언트는 `client_secret_basic`(HTTP Basic) 또는 `client_secret_post`(본문에 시크릿 포함)로 인증합니다. 퍼블릭 클라이언트는 `client_id`만 전송합니다.

[inline-code-attrs-start title = '토큰 요청 cURL 예제'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** 기밀 클라이언트 전용. 대신 HTTP Basic 인증으로 전송할 수 있습니다. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** 전송 시 인가 요청과 일치해야 합니다. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = '토큰 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** fcat_ 접두사. 1시간 동안 유효. **/
    access_token: string
    token_type: 'bearer'
    /** 액세스 토큰 만료까지의 초. 3600. **/
    expires_in: number
    /** fcrt_ 접두사. 발급일로부터 30일 동안 유효. **/
    refresh_token: string
    /** 부여된 스코프, 공백으로 구분. **/
    scope: string
}
[inline-code-end]

오류는 RFC 6749를 따릅니다: `error`와 `error_description`이 포함된 JSON 본문이며, `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target`, `unsupported_grant_type`은 HTTP 400, `invalid_client`는 HTTP 401, 요청 제한 시에는 HTTP 429가 반환됩니다.

### 3단계 - API 호출

액세스 토큰을 bearer 토큰으로 전송합니다. 테넌트는 토큰에 의해 암시되므로 `tenantId`는 선택 사항입니다. 전달하는 경우 토큰과 일치해야 하며, 그렇지 않으면 요청이 실패합니다.

[inline-code-attrs-start title = 'Bearer 토큰 cURL 예제'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me`는 테넌트, 인가한 사용자, 부여된 스코프를 반환하므로 연결 테스트에 적합한 호출입니다. 만료되었거나 취소된 토큰이 포함된 요청은 HTTP 401을 받습니다. 토큰이 보유하지 않은 스코프가 필요한 메서드의 요청은 HTTP 403을 받습니다.

### 4단계 - 갱신

[inline-code-attrs-start title = '갱신 요청 cURL 예제'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** 선택 사항. 원래 부여된 스코프의 일부로 좁힙니다. **/
    scope?: string
    resource?: string
}
[inline-code-end]

응답은 코드 교환과 동일한 형태입니다. 리프레시 토큰은 로테이션됩니다: 각 갱신은 새로운 `refresh_token`을 반환하며, 동시 요청을 위한 30초 유예 기간 후 이전 토큰을 취소합니다. 30초 이상 전에 로테이션된 리프레시 토큰을 제시하면 재생(replay)으로 간주되어 전체 그랜트가 취소됩니다. FastComments가 등록한 파트너 애플리케이션은 로테이션에서 제외되며, 만료일이 30일 연장된 동일한 리프레시 토큰을 받게 됩니다.

갱신 시 인가한 사용자가 여전히 해당 계정에서 API Admin 권한을 보유하는지도 다시 확인합니다. 그렇지 않은 경우 그랜트가 취소되고 응답은 `invalid_grant`입니다.

### 취소

[inline-code-attrs-start title = '취소 요청 cURL 예제'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

리프레시 토큰을 취소하면 동일한 그랜트에서 발급된 모든 액세스 토큰이 취소됩니다. 액세스 토큰을 취소하면 해당 토큰만 취소됩니다. 이 엔드포인트는 RFC 7009에 따라 토큰 발견 여부와 관계없이 빈 JSON 객체와 함께 HTTP 200을 반환합니다.

사용자는 FastComments 대시보드의 **Connected Apps**에서 연결을 취소할 수도 있습니다. 해당 애플리케이션의 모든 토큰은 즉시 작동이 중지됩니다.