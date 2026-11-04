### 인증 API 사용하기 (DefaultAPI)

**중요:**
1. 기본 URL을 설정해야 합니다 (cpp-restsdk 생성기는 OpenAPI 사양에서 이를 읽지 않습니다)
2. 인증이 필요한 요청을 보내기 전에 ApiClient에 API 키를 설정해야 합니다. 그렇지 않으면 요청이 401 오류로 실패합니다.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 필수: 기본 URL 설정 (지역 선택)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // 또는: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 필수: API 키 설정
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 이제 인증된 API 호출을 수행합니다
    return 0;
}
```

### 퍼블릭 API 사용하기 (PublicAPI)

퍼블릭 엔드포인트는 인증이 필요하지 않습니다:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 필수: 기본 URL 설정
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 퍼블릭 API 호출 수행
    return 0;
}
```

### 모더레이션 API 사용하기 (ModerationApi)

`ModerationApi`는 모더레이터 대시보드를 구동합니다. 모든 메서드는 `sso` 매개변수를 받아들이므로, SSO로 인증된 모더레이터를 대신하여 호출이 실행됩니다(토큰 생성 방법은 아래 SSO 섹션을
참조하세요):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 필수: 기본 URL 설정
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // 호출을 인증하기 위해 모더레이터의 SSO 토큰을 전달합니다
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### 일반적인 문제

1. **"URI must contain a hostname" 오류**: ApiClient를 생성하기 전에 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`을 호출했는지 확인하세요. cpp-restsdk 생성기는 OpenAPI 사양에서 서버 URL을 자동으로 읽지 않습니다.
2. **401 "missing-api-key" 오류**: DefaultAPI 인스턴스를 생성하기 전에 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`를 호출했는지 확인하세요.
3. **잘못된 API 클래스 사용**: 서버 측 인증 요청에는 `DefaultApi`를, 클라이언트 측/퍼블릭 요청에는 `PublicApi`를, 모더레이터 대시보드 요청(모더레이터 SSO 토큰으로 인증)에는 `ModerationApi`를 사용하세요.