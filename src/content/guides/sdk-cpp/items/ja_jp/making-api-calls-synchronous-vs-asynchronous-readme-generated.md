このSDKのすべてのAPIメソッドは、C++ REST SDKの `pplx::task<std::shared_ptr<ResponseType>>` を返します。これにより、APIレスポンスの処理方法に柔軟性が生まれます。

### `.get()` を使った同期呼び出し

`.get()` を使用すると、リクエストが完了するまで呼び出しスレッドをブロックし、結果を同期的に取得できます:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 必須パラメータは位置引数で、オプションのパラメータはoptions構造体に入れます
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// .get() を呼び出してブロックし、結果を同期的に取得します
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // HTTPリクエストが完了するまでブロックします

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()` を使った非同期呼び出し

`.then()` を使用すると、コールバックによるノンブロッキングの非同期実行が可能になります:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 必須パラメータは位置引数で、オプションのパラメータはoptions構造体に入れます
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// 非同期のコールバックベースの実行には .then() を使用します
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // リクエスト完了時に非同期で実行されます
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// ブロックせずに即座に実行が続行されます
std::cout << "Request sent, continuing..." << std::endl;
```

### 同期と非同期の選択

選択は、実行環境とアプリケーションのアーキテクチャによって異なります:

**`.get()`(同期ブロッキング)**
- HTTPリクエストが完了するまで呼び出しスレッドをブロックします
- コードの流れがシンプルで、理解しやすいです
- 専用のワーカースレッド、バッチ処理、コマンドラインツールに適しています
- イベントループ、GUIスレッド、シングルスレッドのサーバーには**適していません**

**`.then()`(非同期ノンブロッキング)**
- 即座に返り、リクエスト完了時にコールバックが実行されます
- 呼び出しスレッドをブロックしません
- イベント駆動アーキテクチャ、GUIアプリケーション、シングルスレッドのイベントループに必須です
- 複数の操作をチェーンできます
- 制御フローがより複雑になります

SDKのテストスイートでは `.get()` のみを使用していますが、これはブロッキングが許容されるテスト環境では適切です。