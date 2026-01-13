Всі методи API в цьому SDK повертають `pplx::task<std::shared_ptr<ResponseType>>` з C++ REST SDK. Це дає вам гнучкість у тому, як ви обробляєте відповіді API.

### Синхронні виклики за допомогою `.get()`

Використовуйте `.get()`, щоб блокувати викликаючий потік до завершення запиту та отримати результат синхронно:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Викличіть .get(), щоб блокувати викликаючий потік і отримати результат синхронно
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
).get();  // Блокує, доки HTTP-запит не завершиться

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхронні виклики за допомогою `.then()`

Використовуйте `.then()` для неблокуючого асинхронного виконання з колбеками:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Використовуйте .then() для асинхронного виконання з колбеками
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Це виконується асинхронно, коли запит завершиться
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Виконання продовжується відразу, без блокування
std::cout << "Request sent, continuing..." << std::endl;
```

### Вибір між синхронним та асинхронним

Вибір залежить від вашого середовища виконання та архітектури застосунку:

**`.get()` (Синхронне блокування)**
- Блокує викликаючий потік, доки HTTP-запит не завершиться
- Простіший потік виконання коду, легше розуміти
- Підходить для виділених робочих потоків, пакетної обробки або інструментів командного рядка
- **Не підходить** для циклів подій, GUI-потоків або однопотокових серверів

**`.then()` (Асинхронне, неблокуюче)**
- Повертається негайно, колбек виконується при завершенні запиту
- Не блокує викликаючий потік
- Необхідно для архітектур, орієнтованих на події, GUI-застосунків або однопотокових циклів подій
- Дозволяє ланцюжити кілька операцій
- Складніший потік управління

Набір тестів SDK використовує `.get()` виключно, і це доречно для тестового середовища, де блокування прийнятне.