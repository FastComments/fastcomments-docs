### Kimlik Doğrulamalı API'leri Kullanma (DefaultAPI)

**Önemli:**
1. Temel URL'yi ayarlamalısınız (cpp-restsdk üreticisi bunu OpenAPI spec'ten okumaz)
2. Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamalısınız. Bunu yapmazsanız, istekler 401 hatasıyla başarısız olur.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // GEREKLİ: Temel URL'yi ayarlayın (bölgenizi seçin)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // VEYA: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // GEREKLİ: API anahtarınızı ayarlayın
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Şimdi kimlik doğrulamalı API çağrılarını yapın
    return 0;
}
```

### Genel API'leri Kullanma (PublicAPI)

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

    // Genel API çağrıları yapın
    return 0;
}
```

### Moderatör API'lerini Kullanma (ModerationApi)

`ModerationApi`, moderatör panosunu güçlendirir. Her yöntem bir `sso` parametresi alır; böylece çağrı SSO ile kimlik doğrulamalı bir moderatör adına çalıştırılır (aşağıdaki SSO bölümünde token oluşturma hakkında bilgi bulabilirsiniz):

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

    // Moderatörün SSO token'ını çağrıyı kimlik doğrulamak için geçin
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Ortak Sorunlar

1. **"URI must contain a hostname" hatası**: `ApiClient` oluşturulmadan önce `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` çağrısını yaptığınızdan emin olun. cpp-restsdk üreticisi sunucu URL'sini OpenAPI spec'ten otomatik olarak okumaz.
2. **401 "missing-api-key" hatası**: `DefaultAPI` örneği oluşturulmadan önce `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` çağrısını yaptığınızdan emin olun.
3. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultApi`, istemci/halka açık istekler için `PublicApi` ve moderatör paneli istekleri (moderatör SSO token'ı ile kimlik doğrulamalı) için `ModerationApi` kullanın.