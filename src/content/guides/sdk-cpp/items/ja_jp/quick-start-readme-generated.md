### 認証された API の使用 (DefaultAPI)

**重要:**
1. ベース URL を設定する必要があります（cpp-restsdk ジェネレータは OpenAPI spec から読み取りません）
2. 認証済みリクエストを行う前に ApiClient の ApiClient に API キーを設定する必要があります。設定しないと、リクエストは 401 エラーになります。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベース URL を設定してください（リージョンを選択）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // または: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必須: API キーを設定してください
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // これで認証された API 呼び出しが可能です
    return 0;
}
```

### 公開 API の使用 (PublicAPI)

公開エンドポイントは認証を必要としません:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベース URL を設定してください
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 公開 API を呼び出します
    return 0;
}
```

### よくある問題

1. **"URI must contain a hostname" error**: ApiClient を作成する前に `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` を呼び出していることを確認してください。cpp-restsdk ジェネレータは OpenAPI spec からサーバー URL を自動的に読み取りません。
2. **401 "missing-api-key" error**: DefaultAPI インスタンスを作成する前に `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` を呼び出していることを確認してください。
3. **Wrong API class**: サーバーサイドの認証済みリクエストには `DefaultAPI` を使用し、クライアント側/公開リクエストには `PublicAPI` を使用してください。