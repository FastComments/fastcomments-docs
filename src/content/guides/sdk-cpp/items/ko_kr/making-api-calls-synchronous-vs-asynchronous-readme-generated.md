이 SDK의 모든 API 메서드는 C++ REST SDK의 `pplx::task<std::shared_ptr<ResponseType>>`를 반환합니다. 이를 통해 API 응답을 유연하게 처리할 수 있습니다.

### `.get()`을 사용한 동기 호출

`.get()`을 사용하면 요청이 완료될 때까지 호출 스레드를 차단하고 결과를 동기적으로 가져올 수 있습니다:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 필수 매개변수는 위치 인자로 전달하고, 선택적 매개변수는 options 구조체에 넣습니다
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// .get()을 호출하여 차단하고 결과를 동기적으로 가져옵니다
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // HTTP 요청이 완료될 때까지 차단합니다

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()`을 사용한 비동기 호출

`.then()`을 사용하면 콜백 기반의 논블로킹 비동기 실행이 가능합니다:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// 필수 매개변수는 위치 인자로 전달하고, 선택적 매개변수는 options 구조체에 넣습니다
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// 비동기 콜백 기반 실행을 위해 .then()을 사용합니다
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // 요청이 완료되면 비동기적으로 실행됩니다
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// 차단 없이 즉시 실행이 계속됩니다
std::cout << "Request sent, continuing..." << std::endl;
```

### 동기식과 비동기식 중 선택하기

선택은 런타임 환경과 애플리케이션 아키텍처에 따라 달라집니다:

**`.get()` (동기 블로킹)**
- HTTP 요청이 완료될 때까지 호출 스레드를 차단합니다
- 코드 흐름이 단순하고 이해하기 쉽습니다
- 전용 워커 스레드, 배치 처리, 명령줄 도구에 적합합니다
- 이벤트 루프, GUI 스레드, 단일 스레드 서버에는 **적합하지 않습니다**

**`.then()` (비동기 논블로킹)**
- 즉시 반환되며, 요청이 완료되면 콜백이 실행됩니다
- 호출 스레드를 차단하지 않습니다
- 이벤트 기반 아키텍처, GUI 애플리케이션, 단일 스레드 이벤트 루프에 필수적입니다
- 여러 작업을 체이닝할 수 있습니다
- 제어 흐름이 더 복잡합니다

SDK의 테스트 스위트는 `.get()`만 사용하지만, 이는 블로킹이 허용되는 테스트 환경에서는 적절합니다.