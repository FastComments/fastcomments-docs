### 認証済み API の使用 (DefaultAPI)

**重要:**
1. ベース URL を設定する必要があります（cpp-restsdk ジェネレータは OpenAPI 仕様から自動で読み取らない）
2. 認証されたリクエストを行う前に ApiClient に API キーを設定する必要があります。設定しないと、リクエストは 401 エラーで失敗します。

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベース URL を設定 (リージョンを選択)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // または: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // 必須: API キーを設定
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // これで認証された API 呼び出しを行います
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

    // 必須: ベース URL を設定
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // パブリック API 呼び出しを行います
    return 0;
}
```

### モデレーション API の使用 (ModerationApi)

`ModerationApi` はモデレーターダッシュボードを駆動します。すべてのメソッドは `sso` パラメータを受け取り、SSO 認証されたモデレーターとして呼び出しを実行します（トークンの作成方法は下記 SSO セクションをご参照ください）:

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // 必須: ベース URL を設定
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // モデレーターの SSO トークンを渡して呼び出しを認証します
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### 一般的な問題

1. **"URI must contain a hostname" エラー**: ApiClient を作成する前に `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` を呼び出していることを確認してください。cpp-restsdk ジェネレータは OpenAPI 仕様からサーバー URL を自動的に読み取りません。
2. **401 "missing-api-key" エラー**: DefaultAPI インスタンスを作成する前に `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` を呼び出していることを確認してください。
3. **間違った API クラス**: サーバー側の認証リクエストには `DefaultApi`、クライアント側/公開リクエストには `PublicApi`、モデレーターダッシュボードのリクエスト（モデレーター SSO トークンで認証）には `ModerationApi` を使用してください。