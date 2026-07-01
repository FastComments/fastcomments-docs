All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Синхронные вызовы с `.get()`

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Обязательные параметры позиционные; необязательные идут в структуре options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Вызов .get() для блокировки и получения результата синхронно
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Блокирует, пока HTTP‑запрос не завершится

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхронные вызовы с `.then()`

Use `.then()` for non-blocking asynchronous execution with callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Обязательные параметры позиционные; необязательные идут в структуре options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Используйте .then() для асинхронного выполнения с обратными вызовами
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Этот код выполняется асинхронно после завершения запроса
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Выполнение продолжается сразу же без блокировки
std::cout << "Request sent, continuing..." << std::endl;
```

### Выбор между синхронным и асинхронным

The choice depends on your runtime environment and application architecture:

**`.get()` (Синхронная блокировка)**
- Блокирует вызывающий поток до завершения HTTP‑запроса
- Более простой поток кода, легче понимать
- Подходит для выделенных рабочих потоков, пакетной обработки или консольных утилит
- **Не подходит** для event‑loop’ов, GUI‑потоков или однопоточных серверов

**`.then()` (Асинхронный без блокировки)**
- Возвращает сразу, обратный вызов выполняется после завершения запроса
- Не блокирует вызывающий поток
- Требуется для событийно‑ориентированных архитектур, GUI‑приложений или однопоточных event‑loop’ов
- Позволяет цепочечное выполнение нескольких операций
- Более сложный контрольный поток

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.