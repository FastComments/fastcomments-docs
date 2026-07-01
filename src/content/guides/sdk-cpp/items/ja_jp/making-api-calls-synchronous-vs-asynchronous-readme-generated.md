All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### `.get()` を使用した同期呼び出し

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // HTTP リクエストが完了するまでブロックします

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()` を使用した非同期呼び出し

Use `.then()` for non-blocking asynchronous execution with callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // リクエストが完了したときに非同期で実行されます
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// ブロックせずにすぐに実行が続行されます
std::cout << "Request sent, continuing..." << std::endl;
```

### 同期と非同期の選択

The choice depends on your runtime environment and application architecture:

**`.get()`（同期ブロッキング）**
- 呼び出しスレッドを HTTP リクエストが完了するまでブロックします
- コードフローがシンプルで、理解しやすい
- 専用のワーカースレッド、バッチ処理、コマンドラインツールに適しています
- **イベントループ、GUI スレッド、シングルスレッドサーバーには適さない**

**`.then()`（非同期ノンブロッキング）**
- 即座に戻り、リクエスト完了時にコールバックが実行されます
- 呼び出しスレッドをブロックしません
- イベント駆動型アーキテクチャ、GUI アプリケーション、シングルスレッドのイベントループに必要です
- 複数の操作をチェーンできます
- 制御フローがより複雑になります

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.