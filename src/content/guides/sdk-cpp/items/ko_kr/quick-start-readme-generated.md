### 인증된 API 사용 (DefaultAPI)

**중요:**
1. 기본 URL을 설정해야 합니다 (cpp-restsdk generator는 OpenAPI spec에서 이를 읽지 않습니다)
2. 인증된 요청을 하기 전에 ApiClient에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류로 실패합니다.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL (choose your region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: Set your API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### 공개 API 사용 (PublicAPI)

공개 엔드포인트는 인증이 필요하지 않습니다:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Make public API calls
    return 0;
}
```

### 중재 API 사용 (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Pass the moderator's SSO token to authenticate the call
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    auto response = moderationApi.getCount(
        boost::none,  // textSearch
        boost::none,  // byIPFromComment
        boost::none,  // filter
        boost::none,  // searchFilters
        boost::none,  // demo
        ssoToken      // sso
    ).get();

    return 0;
}
```

### 일반적인 문제

1. **"URI must contain a hostname" error**: ApiClient를 생성하기 전에 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`를 호출했는지 확인하세요. cpp-restsdk generator는 OpenAPI spec에서 서버 URL을 자동으로 읽지 않습니다.
2. **401 "missing-api-key" error**: DefaultAPI 인스턴스를 생성하기 전에 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`를 호출했는지 확인하세요.
3. **잘못된 API 클래스**: 서버 측 인증된 요청에는 `DefaultApi`를 사용하고, 클라이언트 측/공개 요청에는 `PublicApi`를 사용하며, 모더레이터 대시보드 요청에는 `ModerationApi`를 사용하세요 (모더레이터 SSO 토큰으로 인증).
---