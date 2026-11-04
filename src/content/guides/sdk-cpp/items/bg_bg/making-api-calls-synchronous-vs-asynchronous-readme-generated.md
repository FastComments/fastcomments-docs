Всички API методи в този SDK връщат `pplx::task<std::shared_ptr<ResponseType>>` от C++ REST SDK. Това ви дава гъвкавост при обработката на API отговорите.

### Синхронни извиквания с `.get()`

Използвайте `.get()`, за да блокирате извикващата нишка до завършването на заявката и да получите резултата синхронно:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Задължителните параметри са позиционни; незадължителните се подават в options структурата
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Извикайте .get(), за да блокирате и получите резултата синхронно
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Блокира до завършването на HTTP заявката

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхронни извиквания с `.then()`

Използвайте `.then()` за неблокиращо асинхронно изпълнение с callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Задължителните параметри са позиционни; незадължителните се подават в options структурата
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Използвайте .then() за асинхронно изпълнение базирано на callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Това се изпълнява асинхронно, когато заявката завърши
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Изпълнението продължава веднага без блокиране
std::cout << "Request sent, continuing..." << std::endl;
```

### Избор между синхронно и асинхронно изпълнение

Изборът зависи от вашата среда за изпълнение и архитектурата на приложението:

**`.get()` (Синхронно блокиране)**
- Блокира извикващата нишка до завършването на HTTP заявката
- По-прост кодов поток, по-лесен за разбиране
- Подходящо за отделни worker нишки, пакетна обработка или инструменти за командния ред
- **Не е подходящо** за event loops, GUI нишки или еднонишкови сървъри

**`.then()` (Асинхронно неблокиращо)**
- Връща веднага, callback се изпълнява при завършване на заявката
- Не блокира извикващата нишка
- Задължително за event-driven архитектури, GUI приложения или еднонишкови event loops
- Позволява верижно свързване на няколко операции
- По-сложен контролен поток

Тестовият набор на SDK използва изключително `.get()`, но това е подходящо за тестовата среда, където блокирането е приемливо.