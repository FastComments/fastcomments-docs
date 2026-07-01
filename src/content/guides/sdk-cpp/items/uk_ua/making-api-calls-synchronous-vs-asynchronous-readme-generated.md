Усі методи API у цьому SDK повертають `pplx::task<std::shared_ptr<ResponseType>>` з C++ REST SDK. Це дає вам гнучкість у тому, як обробляти відповіді API.

### Синхронні виклики за допомогою `.get()`

Використовуйте `.get()`, щоб блокувати викликаючий потік, доки запит не завершиться, і отримати результат синхронно:

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

### Асинхронні виклики за допомогою `.then()`

Використовуйте `.then()` для неблокуючого асинхронного виконання з зворотними викликами:

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

### Вибір між синхронним та асинхронним

Вибір залежить від вашого середовища виконання та архітектури застосунку:

**`.get()` (Синхронне блокування)**
- Блокує викликаючий потік, доки HTTP‑запит не завершиться
- Простішій потік коду, легше розуміти
- Підходить для виділених робочих потоків, пакетної обробки або інструментів командного рядка
- **Не підходить** для циклів подій, GUI‑потоків або однопотокових серверів

**`.then()` (Асинхронне неблокування)**
- Повертає одразу, зворотний виклик виконується, коли запит завершується
- Не блокує викликаючий потік
- Необхідний для архітектур, орієнтованих на події, GUI‑застосунків або однопотокових циклів подій
- Дозволяє ланцюжити кілька операцій
- Більш складний контроль потоку

Тестовий набір SDK використовує `.get()` виключно, але це підходить для тестового середовища, де блокування прийнятне.