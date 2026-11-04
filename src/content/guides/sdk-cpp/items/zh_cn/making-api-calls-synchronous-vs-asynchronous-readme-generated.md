此 SDK 中的所有 API 方法均返回 C++ REST SDK 的 `pplx::task<std::shared_ptr<ResponseType>>`。这使您可以灵活地处理 API 响应。

### 使用 `.get()` 进行同步调用

使用 `.get()` 阻塞调用线程，直到请求完成并同步获取结果：

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 必需参数按位置传递；可选参数放入 options 结构体中
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// 调用 .get() 以阻塞方式同步获取结果
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // 阻塞直到 HTTP 请求完成

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### 使用 `.then()` 进行异步调用

使用 `.then()` 通过回调进行非阻塞的异步执行：

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 必需参数按位置传递；可选参数放入 options 结构体中
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// 使用 .then() 进行基于回调的异步执行
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // 请求完成时异步执行此代码
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// 执行立即继续，不会阻塞
std::cout << "Request sent, continuing..." << std::endl;
```

### 在同步与异步之间进行选择

选择取决于您的运行环境和应用程序架构：

**`.get()`（同步阻塞）**
- 阻塞调用线程，直到 HTTP 请求完成
- 代码流程更简单，更易于理解
- 适用于专用工作线程、批处理或命令行工具
- **不适用于**事件循环、GUI 线程或单线程服务器

**`.then()`（异步非阻塞）**
- 立即返回，请求完成时执行回调
- 不会阻塞调用线程
- 对于事件驱动架构、GUI 应用程序或单线程事件循环是必需的
- 允许链式执行多个操作
- 控制流程更复杂

该 SDK 的测试套件仅使用 `.get()`，但这对于阻塞可接受的测试环境来说是合适的。