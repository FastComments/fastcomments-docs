All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Синхронни извиквания с `.get()`

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

### Асинхронни извиквания с `.then()`

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

### Избор между синхронно и асинхронно

The choice depends on your runtime environment and application architecture:

**`.get()` (Синхронно блокиране)**
- Blocks the calling thread until the HTTP request completes → Блокира извикващата нишка, докато HTTP заявката завърши
- Simpler code flow, easier to reason about → По‑опростен поток на кода, по‑лесен за разбиране
- Suitable for dedicated worker threads, batch processing, or command-line tools → Подходящ за специализирани работни нишки, пакетна обработка или инструменти за команден ред
- **Not suitable** for event loops, GUI threads, or single-threaded servers → **Не е подходящ** за събитийни цикли, GUI нишки или еднонишкови сървъри

**`.then()` (Асинхронно неблокиращо)**
- Returns immediately, callback executes when request completes → Връща се незабавно, обратното извикване се изпълнява, когато заявката завърши
- Does not block the calling thread → Не блокира извикващата нишка
- Required for event-driven architectures, GUI applications, or single-threaded event loops → Необходимо за събитийно‑ориентирани архитектури, GUI приложения или еднонишкови събитийни цикли
- Allows chaining multiple operations → Позволява свързване на множество операции
- More complex control flow → По‑сложен контролен поток

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.