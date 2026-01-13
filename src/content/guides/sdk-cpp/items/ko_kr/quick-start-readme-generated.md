### 인증된 API 사용 (DefaultAPI)

**중요:**
1. 기본 베이스 URL을 설정해야 합니다 (cpp-restsdk generator는 OpenAPI 스펙에서 이를 읽어오지 않습니다)
2. 인증된 요청을 하기 전에 ApiClient에 API 키를 설정해야 합니다. 설정하지 않으면 요청이 401 오류로 실패합니다.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 필수: 기본 URL을 설정하세요 (지역을 선택)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // 또는: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 필수: API 키를 설정하세요
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // 이제 인증된 API 호출을 수행하세요
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

    // 필수: 기본 URL을 설정하세요
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 공개 API 호출을 수행하세요
    return 0;
}
```

### 일반적인 문제

1. **"URI must contain a hostname" error**: ApiClient를 생성하기 전에 `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))`를 호출했는지 확인하세요. cpp-restsdk generator는 OpenAPI 스펙에서 서버 URL을 자동으로 읽어오지 않습니다.
2. **401 "missing-api-key" error**: DefaultAPI 인스턴스를 생성하기 전에 `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))`를 호출했는지 확인하세요.
3. **Wrong API class**: 서버 측 인증 요청에는 `DefaultAPI`를 사용하고, 클라이언트 측/공개 요청에는 `PublicAPI`를 사용하세요.