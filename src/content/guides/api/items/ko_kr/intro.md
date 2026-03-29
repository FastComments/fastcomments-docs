### FastComments API

FastComments는 다양한 리소스와 상호작용하기 위한 API를 제공합니다. 우리 플랫폼과의 통합을 구축하거나, 자체 클라이언트를 직접 만들어보세요!

이 문서에서는 API가 지원하는 모든 리소스를 요청 및 응답 타입과 함께 문서화하여 확인할 수 있습니다.

엔터프라이즈 고객의 경우 모든 API 접근은 감사 로그에 기록됩니다.

### Generated SDKs

FastComments는 이제 코드로부터 [API Spec](https://fastcomments.com/js/swagger.json)을 생성합니다 (아직 완전하진 않지만 많은 API가 포함되어 있습니다).

또한 다음과 같은 인기 언어용 SDK도 제공합니다:

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

### Authentication

API는 [API 키](https://fastcomments.com/auth/my-account/api-secret)를 `X-API-KEY` 헤더 또는 `API_KEY` 쿼리 매개변수로 전달하여 인증합니다. 또한 API 호출을 위해 `tenantId`가 필요합니다. 이 값은 API 키와 동일한 페이지에서 확인할 수 있습니다.

### Security Note

이 라우트들은 **서버**에서 호출되도록 설계되었습니다. __절대__ 브라우저에서 호출하지 마세요. 이렇게 하면 API 키가 노출되어 페이지 소스 코드를 볼 수 있는 누구에게나 계정 전체에 대한 액세스 권한이 제공됩니다!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### 자신의 작성 내용 읽기

FastComments는 Active-Active 가용성을 제공합니다. 데이터센터에서 보내는 요청은 자동으로 귀하의 위치에 가장 가까운 [the nearest point of presence](https://sophon.fastcomments.com/)로 라우팅됩니다. 이는 자동으로 이루어지며, 보통은 자신의 쓰기(작성)를 읽는 동작을 관찰할 수 있습니다. 자신의 작성 내용을 확실히 읽고 싶다면 해당 리전을 API 호스트로 사용하여 요청을 특정 리전에 고정(pinning)할 수 있습니다(그러나 대부분의 통합에서는 보통 필요하지 않습니다):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

참고: 이렇게 할 경우 과거에 엔트리포인트 노드를 더 이상 사용하지 않게(deprecated)하고 전환 시 새 이름을 사용한 사례가 있으므로 페일백(fallback)을 정의하는 것이 좋습니다.