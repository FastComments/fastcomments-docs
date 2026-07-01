All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

此 SDK 中的所有 API 方法都返回来自 C++ REST SDK 的 `pplx::task<std::shared_ptr<ResponseType>>`。这为您处理 API 响应提供了灵活性。

### 使用 `.get()` 的同步调用

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
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### 使用 `.then()` 的异步调用

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
    // This runs asynchronously when the request completes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### 在同步和异步之间的选择

The choice depends on your runtime environment and application architecture:

**`.get()` (Synchronous blocking)**
- Blocks the calling thread until the HTTP request completes
  → 阻塞调用线程，直到 HTTP 请求完成
- Simpler code flow, easier to reason about
  → 代码流程更简单，易于理解
- Suitable for dedicated worker threads, batch processing, or command-line tools
  → 适用于专用工作线程、批处理或命令行工具
- **Not suitable** for event loops, GUI threads, or single-threaded servers
  → **不适合** 事件循环、GUI 线程或单线程服务器

**`.then()` (Asynchronous non-blocking)**
- Returns immediately, callback executes when request completes
  → 立即返回，请求完成时执行回调
- Does not block the calling thread
  → 不阻塞调用线程
- Required for event-driven architectures, GUI applications, or single-threaded event loops
  → 在事件驱动架构、GUI 应用程序或单线程事件循环中是必需的
- Allows chaining multiple operations
  → 允许链式调用多个操作
- More complex control flow
  → 控制流更复杂

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.

SDK 的测试套件仅使用 `.get()`，但这在阻塞是可接受的测试环境中是合适的。