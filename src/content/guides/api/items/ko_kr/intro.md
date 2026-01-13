### FastComments API

FastComments는 여러 리소스와 상호 작용하기 위한 API를 제공합니다. 우리 플랫폼과 통합을 구축하거나, 직접 클라이언트를 만들어 보세요!

이 문서에서는 API가 지원하는 모든 리소스가 요청 및 응답 타입과 함께 문서화되어 있습니다.

엔터프라이즈 고객의 경우 모든 API 접근이 감사 로그에 기록됩니다.

### 생성된 SDKs

FastComments는 이제 코드에서 [API Spec](https://fastcomments.com/js/swagger.json)을 생성합니다 (아직 완전하지는 않지만 많은 API가 포함되어 있습니다).

또한 인기 있는 언어용 SDK도 제공됩니다:

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

API는 [api key](https://fastcomments.com/auth/my-account/api-secret)를 `X-API-KEY` 헤더 또는 `API_KEY` 쿼리 매개변수로 전달하여 인증합니다. API 호출을 위해 `tenantId`도 필요합니다. 이는 api key가 있는 동일한 페이지에서 가져올 수 있습니다.

### 보안 참고

이 경로들은 **서버**에서 호출되도록 되어 있습니다. 브라우저에서 __절대__ 호출하지 마세요. 이렇게 하면 API 키가 노출됩니다 - 페이지의 소스 코드를 볼 수 있는 누구에게나 계정에 대한 전체 접근 권한이 부여됩니다!

#### 인증 옵션 1 - 헤더

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### 인증 옵션 2 - 쿼리 매개변수

- Query Param: `API_KEY`
- Query Param: `tenantId`

---