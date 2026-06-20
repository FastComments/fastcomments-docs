### Kimlik Doğrulamalı API'lerin Kullanımı (DefaultAPI)

**Önemli:**
1. Taban URL'yi ayarlamanız gerekir (cpp-restsdk jeneratörü bunu OpenAPI spesifikasyonundan okumaz)
2. Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamanız gerekir. Yapmazsanız, istekler 401 hatası ile başarısız olur.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // GEREKLİ: Temel URL'yi ayarlayın (bölgenizi seçin)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // ABD
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // AB

    // GEREKLİ: API anahtarınızı ayarlayın
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Artık kimlik doğrulamalı API çağrıları yapın
    return 0;
}
```

### Genel Halka Açık API'lerin Kullanımı (PublicAPI)

Genel uç noktalar kimlik doğrulama gerektirmez:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // GEREKLİ: Temel URL'yi ayarlayın
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Genel halka açık API çağrıları yapın
    return 0;
}
```

### Moderasyon API'lerinin Kullanımı (ModerationApi)

`ModerationApi` moderatör panelini sağlar. Her yöntem bir `sso` parametresini kabul eder, böylece çağrı bir SSO ile kimlik doğrulanmış moderatör adına çalışır (bir belirteç oluşturma hakkında bilgi için aşağıdaki SSO bölümüne bakın):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // GEREKLİ: Temel URL'yi ayarlayın
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Çağrıyı kimlik doğrulamak için moderatörün SSO belirtecini iletin
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

### Yaygın Sorunlar

1. **"URI must contain a hostname" hatası**: ApiClient'ı oluşturmadan önce `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` çağrısını yaptığınızdan emin olun. cpp-restsdk jeneratörü sunucu URL'sini OpenAPI spec'ten otomatik olarak okumaz.
2. **401 "missing-api-key" hatası**: DefaultAPI örneğini oluşturmadan önce `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` çağrısını yaptığınızdan emin olun.
3. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`'yi, istemci tarafı/genel istekler için `PublicApi`'yi ve moderatör paneli istekleri için (moderator SSO belirteci ile kimlik doğrulanan) `ModerationApi`'yi kullanın.