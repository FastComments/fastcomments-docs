### Using Authenticated APIs (DefaultAPI)

**Important:**
1. 베이스 URL을 설정해야 합니다 (cpp‑restsdk 생성기는 OpenAPI 사양에서 이를 읽어들이지 않습니다)
2. 인증된 요청을 하기 전에 ApiClient에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류와 함께 실패합니다.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: 베이스 URL 설정 (지역 선택)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: API 키 설정
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 이제 인증된 API 호출을 수행합니다
    return 0;
}
```

### Using Public APIs (PublicAPI)

Public endpoints don't require authentication:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: 베이스 URL 설정
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 공용 API 호출 수행
    return 0;
}
```

### Using Moderation APIs (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO‑authenticated moderator (see the SSO section below for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: 베이스 URL 설정
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // 중재자의 SSO 토큰을 전달하여 호출을 인증합니다
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Common Issues

1. **"URI must contain a hostname" error**: ApiClient를 생성하기 전에 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`를 호출했는지 확인하십시오. cpp‑restsdk 생성기는 OpenAPI 사양에서 서버 URL을 자동으로 읽어들이지 않습니다.
2. **401 "missing-api-key" error**: DefaultAPI 인스턴스를 생성하기 전에 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`를 호출했는지 확인하십시오.
3. **Wrong API class**: 서버‑사이드 인증 요청에는 `DefaultApi`를, 클라이언트‑사이드/공용 요청에는 `PublicApi`를, 중재자 대시보드 요청(중재자 SSO 토큰으로 인증)에는 `ModerationApi`를 사용하십시오.