### Kimlik Doğrulamalı API'leri Kullanma (DefaultAPI)

**Önemli:**
1. Temel URL'i ayarlamalısınız (cpp-restsdk jeneratörü bunu OpenAPI spesifikasyonundan okumaz)
2. Kimlik doğrulamalı istekler yapmadan önce ApiClient üzerinde API anahtarınızı ayarlamalısınız. Ayarlamazsanız, istekler 401 hatası ile başarısız olur.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // GEREKLİ: Temel URL'i ayarlayın (bölgenizi seçin)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // VEYA: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // GEREKLİ: API anahtarınızı ayarlayın
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Artık kimlik doğrulamalı API çağrıları yapın
    return 0;
}
```

### Herkese Açık API'leri Kullanma (PublicAPI)

Herkese açık uç noktalar kimlik doğrulama gerektirmez:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // GEREKLİ: Temel URL'i ayarlayın
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Genel API çağrıları yapın
    return 0;
}
```

### Yaygın Sorunlar

1. **"URI must contain a hostname" hatası**: ApiClient oluşturulmadan önce `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` çağrısını yaptığınızdan emin olun. cpp-restsdk jeneratörü sunucu URL'sini OpenAPI spesifikasyonundan otomatik olarak okumaz.
2. **401 "missing-api-key" hatası**: DefaultAPI örneği oluşturulmadan önce `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` çağrısını yaptığınızdan emin olun.
3. **Yanlış API sınıfı**: Sunucu tarafı kimlik doğrulamalı istekler için `DefaultAPI`, istemci tarafı/genel istekler için `PublicAPI` kullanın.