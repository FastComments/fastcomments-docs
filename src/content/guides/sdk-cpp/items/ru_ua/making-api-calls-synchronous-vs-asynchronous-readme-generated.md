Все методы API в этом SDK возвращают `pplx::task<std::shared_ptr<ResponseType>>` из C++ REST SDK. Это даёт вам гибкость в том, как обрабатывать ответы API.

### Синхронные вызовы с `.get()`

Используйте `.get()`, чтобы блокировать вызывающий поток до завершения запроса и получить результат синхронно:

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
).get();  // Блокирует до завершения HTTP-запроса

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchronous Calls with `.then()`

Use `.then()` for non-blocking asynchronous execution with callbacks:

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

### Выбор между синхронным и асинхронным

Выбор зависит от вашей среды выполнения и архитектуры приложения:

**`.get()` (Синхронная блокировка)**
- Блокирует вызывающий поток до завершения HTTP-запроса
- Проще поток выполнения кода, его легче анализировать
- Подходит для выделенных рабочих потоков, пакетной обработки или консольных утилит
- **Не подходит** для циклов событий, GUI-потоков или однопоточных серверов

**`.then()` (Асинхронный неблокирующий)**
- Возвращает управление сразу, обратный вызов выполняется после завершения запроса
- Не блокирует вызывающий поток
- Требуется для событийно-ориентированных архитектур, GUI-приложений или однопоточных циклов событий
- Позволяет связывать несколько операций в цепочку
- Более сложный поток управления

Набор тестов SDK использует только `.get()`, что подходит для тестовой среды, где блокировка допустима.