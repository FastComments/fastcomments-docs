### Коришћење аутентификованих API‑ја (DefaultAPI)

**Важно:**
1. Морате поставити базни URL (генератор cpp-restsdk не чита то из OpenAPI спецификације)
2. Морате поставити ваш API кључ у ApiClient пре него што направите аутентификоване захтеве. Ако то не урадите, захтеви ће пропасти са грешком 401.

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

### Коришћење јавних API‑ја (PublicAPI)

Јавни крајњи поени не захтевају аутентификацију:

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

### Коришћење модерацијских API‑ја (ModerationApi)

„ModerationApi“ покреће контролни панел модератора. Свака метода прихвата параметар `sso` тако да позив ради у име SSO‑аутентификованог модератора (погледајте SSO секцију испод за то како се креира токен):

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

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Уобичајени проблеми

1. **„URI must contain a hostname“ грешка**: Уверите се да позовете `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` пре него што креирате ApiClient. Генератор cpp-restsdk не чита аутоматски URL сервера из OpenAPI спецификације.
2. **401 „missing-api-key“ грешка**: Уверите се да позовете `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` пре него што креирате DefaultAPI инстанцу.
3. **Погрешна API класа**: Користите `DefaultApi` за серверске аутентификоване захтеве, `PublicApi` за клијентске/јавне захтеве, и `ModerationApi` за захтеве контролног панела модератора (аутентификоване модераторским SSO токеном).