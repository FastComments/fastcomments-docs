---
### FastComments API

FastComments는 다양한 리소스와 상호 작용할 수 있는 API를 제공합니다. 우리 플랫폼과 통합을 구축하거나 직접 클라이언트를 만들 수도 있습니다!

이 문서에서는 API가 지원하는 모든 리소스를 요청 및 응답 유형과 함께 확인할 수 있습니다.

엔터프라이즈 고객의 경우, 모든 API 접근이 감사 로그에 기록됩니다.

### 생성된 SDK

FastComments는 이제 코드에서 [API 사양](https://fastcomments.com/js/swagger.json)을 생성합니다(아직 완전하지 않지만 많은 API를 포함합니다).

또한 현재 인기 있는 언어용 SDK도 제공됩니다:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### 인증

API는 [API 키](https://fastcomments.com/auth/my-account/api-secret)를 `X-API-KEY` 헤더 또는 `API_KEY` 쿼리 매개변수로 전달하여 인증합니다. API 호출을 위해 `tenantId`도 필요합니다. 이는 API 키와 같은 페이지에서 확인할 수 있습니다.

### 보안 주의사항

이 경로는 **서버**에서 호출하도록 설계되었습니다. __절대__ 브라우저에서 호출하지 마세요. 이렇게 하면 API 키가 노출되어 페이지 소스 코드를 볼 수 있는 누구든지 계정에 대한 전체 접근 권한을 얻게 됩니다!

#### 인증 옵션 1 - 헤더

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### 인증 옵션 2 - 쿼리 매개변수

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### 인증 옵션 3 - OAuth 베어러 토큰

- Header: `Authorization: Bearer fcat_...`

[MCP 서버](https://docs.fastcomments.com/guide-llm-kit.html)를 통해 연결하는 애플리케이션은 API 키 대신 OAuth를 통해 토큰을 얻습니다. 해당 토큰은 여기의 모든 엔드포인트에서 작동합니다. 토큰에 테넌트가 내포되어 있으므로 `tenantId`는 선택 사항이지만 제공되는 경우 토큰과 일치해야 합니다. `GET` 요청은 `read` 스코프가 필요하고, 다른 모든 메서드는 `write` 스코프가 필요합니다. 탐색은 `https://fastcomments.com/.well-known/oauth-authorization-server`에서 시작됩니다.

### 자신의 쓰기 읽기

FastComments는 Active-Active 가용성을 제공합니다. 데이터센터에서 오는 요청은 여러분에게 가장 가까운 [지점](https://sophon.fastcomments.com/)으로 라우팅됩니다. 이는 자동이며 일반적으로 읽기-쓰기 일관성을 관찰할 수 있습니다. 자신의 쓰기를 확실히 읽고 싶다면 해당 지역을 API 호스트로 지정하여 요청을 특정 지역에 고정할 수 있습니다(대부분의 통합에서는 보통 필요하지 않습니다).

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

이렇게 할 경우, 과거에 진입점 노드를 폐기하고 전환을 위한 새로운 이름을 사용했으므로 대체 경로를 정의하는 것이 좋습니다.

---