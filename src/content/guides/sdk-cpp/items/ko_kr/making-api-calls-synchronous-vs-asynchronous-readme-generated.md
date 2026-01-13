이 SDK의 모든 API 메서드는 C++ REST SDK의 `pplx::task<std::shared_ptr<ResponseType>>`를 반환합니다. 이는 API 응답을 처리하는 방법에 대해 유연성을 제공합니다.

### `.get()`를 사용하는 동기 호출

요청이 완료될 때까지 호출 스레드를 차단하고 결과를 동기적으로 가져오려면 `.get()`을 사용하십시오:

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
).get();  // HTTP 요청이 완료될 때까지 차단합니다

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()`를 사용하는 비동기 호출

콜백을 사용한 논블로킹 비동기 실행을 위해 `.then()`을 사용하십시오:

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

### 동기와 비동기 중 선택하기

선택은 런타임 환경과 애플리케이션 아키텍처에 따라 다릅니다:

**`.get()` (동기 블로킹)**
- HTTP 요청이 완료될 때까지 호출 스레드를 차단합니다
- 코드 흐름이 단순하고 이해하기 쉽습니다
- 전용 워커 스레드, 배치 처리 또는 명령줄 도구에 적합합니다
- **적합하지 않음**: 이벤트 루프, GUI 스레드 또는 단일 스레드 서버

**`.then()` (비동기 논블로킹)**
- 즉시 반환되며, 요청이 완료되면 콜백이 실행됩니다
- 호출 스레드를 차단하지 않습니다
- 이벤트 기반 아키텍처, GUI 애플리케이션 또는 단일 스레드 이벤트 루프에서 필요합니다
- 여러 작업을 체인할 수 있습니다
- 제어 흐름이 더 복잡합니다

이 SDK의 테스트 스위트는 `.get()`만 사용하지만, 이는 블로킹이 허용되는 테스트 환경에서는 적절합니다.