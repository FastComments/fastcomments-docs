Все методы API в этой SDK возвращают `pplx::task<std::shared_ptr<ResponseType>>` из C++ REST SDK. Это даёт вам гибкость в том, как обрабатывать ответы API.

### Synchronous Calls with `.get()`

Используйте `.get()`, чтобы заблокировать вызывающий поток до завершения запроса и получить результат синхронно:

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
    boost::none,  // страница
    boost::none,  // лимит
    boost::none,  // смещение
    boost::none,  // как дерево
    boost::none,  // пропустить дочерние
    boost::none,  // лимит дочерних
    boost::none,  // макс. глубина дерева
    utility::conversions::to_string_t("your-url-id"),  // идентификатор URL
    boost::none,  // идентификатор пользователя
    boost::none,  // анонимный идентификатор пользователя
    boost::none,  // контекстный идентификатор пользователя
    boost::none,  // хештег
    boost::none,  // идентификатор родителя
    boost::none   // направление
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchronous Calls with `.then()`

Используйте `.then()` для неблокирующего асинхронного выполнения с колбэками:

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

### Choosing Between Synchronous and Asynchronous

Выбор зависит от вашей среды выполнения и архитектуры приложения:

**`.get()` (Synchronous blocking)**
- Блокирует вызывающий поток до завершения HTTP-запроса
- Проще поток выполнения, легче понять
- Подходит для выделенных рабочих потоков, пакетной обработки или командной строки
- **Не подходит** для циклов событий, GUI-потоков или однопоточных серверов

**`.then()` (Asynchronous non-blocking)**
- Возвращает управление сразу, колбэк выполняется при завершении запроса
- Не блокирует вызывающий поток
- Необходим для событийно-ориентированной архитектуры, GUI-приложений или однопоточных циклов событий
- Позволяет цепочить несколько операций
- Более сложный поток управления

Набор тестов SDK использует только `.get()`, что уместно в тестовой среде, где блокировка допустима.