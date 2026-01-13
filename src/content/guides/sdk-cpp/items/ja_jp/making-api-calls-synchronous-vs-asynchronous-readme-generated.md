このSDKのすべてのAPIメソッドはC++ REST SDKの`pplx::task<std::shared_ptr<ResponseType>>`を返します。これにより、APIレスポンスの処理方法に柔軟性が得られます。

### `.get()` を使った同期呼び出し

`.get()` を使用して、リクエストが完了するまで呼び出し元のスレッドをブロックし、結果を同期的に取得します:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // page
    boost::none,  // limit
    boost::none,  // skip
    boost::none,  // asTree
    boost::none,  // skipChildren
    boost::none,  // limitChildren
    boost::none,  // maxTreeDepth
    utility::conversions::to_string_t("your-url-id"),  // urlId
    boost::none,  // userId
    boost::none,  // anonUserId
    boost::none,  // contextUserId
    boost::none,  // hashTag
    boost::none,  // parentId
    boost::none   // direction
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()` を使った非同期呼び出し

コールバックによるノンブロッキングの非同期実行には`.then()`を使用します:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // This runs asynchronously when the request completes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### 同期と非同期の選択

選択はランタイム環境とアプリケーションのアーキテクチャによります:

**`.get()`（同期ブロッキング）**
- HTTPリクエストが完了するまで呼び出しスレッドをブロックする
- コードの流れが単純で、理解しやすい
- 専用のワーカースレッド、バッチ処理、またはコマンドラインツールに適している
- **イベントループ、GUIスレッド、または単一スレッドのサーバーには適していない**

**`.then()`（非同期ノンブロッキング）**
- 即座に戻り、リクエスト完了時にコールバックが実行される
- 呼び出しスレッドをブロックしない
- イベント駆動型アーキテクチャ、GUIアプリケーション、または単一スレッドのイベントループでは必要
- 複数の操作のチェーンを可能にする
- 制御フローがより複雑になる

このSDKのテストスイートは`.get()`のみを使用していますが、これはブロッキングが許容されるテスト環境では適切です。