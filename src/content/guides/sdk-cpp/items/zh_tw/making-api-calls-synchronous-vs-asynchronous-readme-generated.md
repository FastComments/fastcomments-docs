此 SDK 中的所有 API 方法皆回傳來自 C++ REST SDK 的 `pplx::task<std::shared_ptr<ResponseType>>`。這讓您在處理 API 回應時有彈性。

### 使用 `.get()` 的同步呼叫

使用 `.get()` 會阻塞呼叫執行緒直到請求完成，並同步取得結果：

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
    boost::none,  // 頁碼
    boost::none,  // 每頁限制
    boost::none,  // 跳過數量
    boost::none,  // 以樹狀回傳
    boost::none,  // 跳過子項
    boost::none,  // 子項限制
    boost::none,  // 樹狀最大深度
    utility::conversions::to_string_t("your-url-id"),  // urlId
    boost::none,  // userId
    boost::none,  // anonUserId
    boost::none,  // contextUserId
    boost::none,  // hashTag
    boost::none,  // parentId
    boost::none   // direction
).get();  // 會阻塞直到 HTTP 請求完成

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### 使用 `.then()` 的非同步呼叫

使用 `.then()` 進行非阻塞的非同步執行並使用回呼：

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
    // 當請求完成時會非同步執行
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// 程式會立即繼續執行，不會被阻塞
std::cout << "Request sent, continuing..." << std::endl;
```

### 在同步與非同步之間的選擇

選擇取決於您的執行環境與應用程式架構：

**`.get()` (同步阻塞)**
- 會阻塞呼叫執行緒直到 HTTP 請求完成
- 程式流程較簡單，較易理解
- 適用於專用工作執行緒、批次處理或命令列工具
- **不適用於** 事件迴圈、GUI 執行緒或單執行緒伺服器

**`.then()` (非同步非阻塞)**
- 立即返回，回呼於請求完成時執行
- 不會阻塞呼叫執行緒
- 適用於事件驅動架構、GUI 應用程式或單執行緒事件迴圈
- 允許串接多個操作
- 程式流程較複雜

該 SDK 的測試套件僅使用 `.get()`，但在允許阻塞的測試環境中這是合適的。