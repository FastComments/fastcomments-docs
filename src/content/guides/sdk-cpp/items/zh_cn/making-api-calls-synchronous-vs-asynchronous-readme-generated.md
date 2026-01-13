所有此 SDK 中的 API 方法都返回来自 C++ REST SDK 的 `pplx::task<std::shared_ptr<ResponseType>>`。这使您在处理 API 响应时具有更大的灵活性。

### 使用 `.get()` 的同步调用

使用 `.get()` 阻塞调用线程，直到请求完成并同步检索结果：

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 调用 .get() 来阻塞并同步获取结果
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
).get();  // 阻塞直到 HTTP 请求完成

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### 使用 `.then()` 的异步调用

使用 `.then()` 进行非阻塞的异步执行并使用回调：

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 使用 .then() 进行基于回调的异步执行
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // 当请求完成时异步运行此代码
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// 执行会立即继续而不会阻塞
std::cout << "Request sent, continuing..." << std::endl;
```

### 在同步与异步之间进行选择

选择取决于您的运行时环境和应用程序架构：

**`.get()` (同步阻塞)**
- 阻塞调用线程直到 HTTP 请求完成
- 代码流程更简单，更易于理解
- 适用于专用工作线程、批处理或命令行工具
- 不适用于事件循环、GUI 线程或单线程服务器

**`.then()` (异步非阻塞)**
- 立即返回，回调在请求完成后执行
- 不会阻塞调用线程
- 适用于事件驱动架构、GUI 应用或单线程事件循环
- 允许链式执行多个操作
- 控制流更复杂

SDK 的测试套件仅使用 `.get()`，但这适用于测试环境，在该环境中阻塞是可以接受的。