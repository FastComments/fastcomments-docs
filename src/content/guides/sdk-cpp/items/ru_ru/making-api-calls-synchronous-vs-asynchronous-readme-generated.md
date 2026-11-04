Все методы API в этом SDK возвращают `pplx::task<std::shared_ptr<ResponseType>>` из C++ REST SDK. Это даёт вам гибкость в обработке ответов API.

### Синхронные вызовы с `.get()`

Используйте `.get()`, чтобы заблокировать вызывающий поток до завершения запроса и получить результат синхронно:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Обязательные параметры передаются позиционно; необязательные — в структуре options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Вызываем .get(), чтобы заблокировать выполнение и получить результат синхронно
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Блокирует до завершения HTTP-запроса

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхронные вызовы с `.then()`

Используйте `.then()` для неблокирующего асинхронного выполнения с колбэками:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Обязательные параметры передаются позиционно; необязательные — в структуре options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Используем .then() для асинхронного выполнения на основе колбэков
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Этот код выполняется асинхронно после завершения запроса
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Выполнение продолжается сразу, без блокировки
std::cout << "Request sent, continuing..." << std::endl;
```

### Выбор между синхронным и асинхронным вариантами

Выбор зависит от вашей среды выполнения и архитектуры приложения:

**`.get()` (синхронная блокировка)**
- Блокирует вызывающий поток до завершения HTTP-запроса
- Более простой поток кода, легче для понимания
- Подходит для выделенных рабочих потоков, пакетной обработки или утилит командной строки
- **Не подходит** для циклов событий, GUI-потоков или однопоточных серверов

**`.then()` (асинхронное неблокирующее выполнение)**
- Возвращается немедленно, колбэк выполняется после завершения запроса
- Не блокирует вызывающий поток
- Необходимо для событийно-ориентированных архитектур, GUI-приложений или однопоточных циклов событий
- Позволяет объединять несколько операций в цепочку
- Более сложный поток управления

Набор тестов SDK использует исключительно `.get()`, но это уместно в тестовой среде, где блокировка допустима.