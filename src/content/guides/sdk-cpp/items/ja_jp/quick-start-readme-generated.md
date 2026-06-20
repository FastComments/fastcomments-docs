### 認証済みAPIの使用 (DefaultAPI)

**重要:**
1. ベースURLを設定する必要があります（cpp-restsdk ジェネレータは OpenAPI スペックから読み取りません）
2. 認証付きリクエストを行う前に ApiClient に API キーを設定する必要があります。設定していないとリクエストは 401 エラーになります。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベースURLを設定してください（リージョンを選択）
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // または: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必須: APIキーを設定してください
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // これで認証されたAPI呼び出しを行えます
    return 0;
}
```

### 公開APIの使用 (PublicAPI)

公開エンドポイントは認証を必要としません:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベースURLを設定してください
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // 公開APIの呼び出しを行います
    return 0;
}
```

### Moderation API の使用 (ModerationApi)

`ModerationApi` はモデレーターダッシュボードを支えています。すべてのメソッドは `sso` パラメータを受け取り、その呼び出しが SSO 認証されたモデレーターを代表して実行されるようになります（トークンの作成方法は下の SSO セクションを参照してください）:

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベースURLを設定してください
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // 呼び出しを認証するためにモデレーターのSSOトークンを渡します
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

### よくある問題

1. **"URI must contain a hostname" error**: ApiClient を作成する前に `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` を呼び出していることを確認してください。cpp-restsdk ジェネレータは OpenAPI スペックからサーバーURLを自動的に読み取りません。
2. **401 "missing-api-key" error**: DefaultAPI インスタンスを作成する前に `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` を呼び出していることを確認してください。
3. **Wrong API class**: サーバー側での認証済みリクエストには `DefaultApi` を、クライアント側/公開リクエストには `PublicApi` を、モデレーターダッシュボード向けのリクエスト（モデレーターの SSO トークンで認証）には `ModerationApi` を使用してください。