此 SDK 中的所有 API 方法都會從 C++ REST SDK 回傳 `pplx::task<std::shared_ptr<ResponseType>>`。這讓您可以靈活地處理 API 回應。

### 使用 `.get()` 進行同步呼叫

使用 `.get()` 來阻塞呼叫執行緒，直到請求完成並同步取得結果：

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 必要參數以位置參數傳入；選用參數則放在 options 結構中
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// 呼叫 .get() 以阻塞方式同步取得結果
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // 阻塞直到 HTTP 請求完成

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### 使用 `.then()` 進行非同步呼叫

使用 `.then()` 進行非阻塞的非同步執行與回呼處理：

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 必要參數以位置參數傳入；選用參數則放在 options 結構中
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// 使用 .then() 進行基於回呼的非同步執行
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // 當請求完成時，此段程式碼會以非同步方式執行
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// 執行會立即繼續，不會阻塞
std::cout << "Request sent, continuing..." << std::endl;
```

### 在同步與非同步之間做選擇

選擇取決於您的執行環境與應用程式架構：

**`.get()`（同步阻塞）**
- 阻塞呼叫執行緒，直到 HTTP 請求完成
- 程式流程較簡單，較容易理解
- 適合專用的工作者執行緒、批次處理或命令列工具
- **不適合**事件迴圈、GUI 執行緒或單執行緒伺服器

**`.then()`（非同步非阻塞）**
- 立即回傳，回呼會在請求完成時執行
- 不會阻塞呼叫執行緒
- 對於事件驅動架構、GUI 應用程式或單執行緒事件迴圈為必要選擇
- 允許鏈接多個操作
- 控制流程較為複雜

此 SDK 的測試套件僅使用 `.get()`，但這在阻塞可被接受的測試環境中是合適的。