Всички API методи в това SDK връщат `pplx::task<std::shared_ptr<ResponseType>>` от C++ REST SDK. Това ви дава гъвкавост в начина, по който обработвате отговорите от API-то.

### Синхронни повиквания с `.get()`

Използвайте `.get()` за да блокирате извикващия нишка, докато заявката не завърши, и да получите резултата синхронно:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Извикайте .get() за да блокирате и да получите резултата синхронно
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
).get();  // Блокира, докато HTTP заявката не приключи

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхронни повиквания с `.then()`

Използвайте `.then()` за неблокиращо асинхронно изпълнение с callback-и:

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
    // Това се изпълнява асинхронно, когато заявката завърши
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Изпълнението продължава веднага без блокиране
std::cout << "Request sent, continuing..." << std::endl;
```

### Избор между синхронно и асинхронно

Изборът зависи от средата за изпълнение и архитектурата на приложението:

**`.get()` (Синхронно блокиращо)**
- Блокира извикващата нишка, докато HTTP заявката не завърши
- По-прост поток на кода, по-лесно за разбиране
- Подходящо за отделни работни нишки, пакетна обработка или командни инструменти
- **Не е подходящо** за цикли на събития, GUI нишки или еднонишкови сървъри

**`.then()` (Асинхронно неблокиращо)**
- Връща веднага, функцията за обратно извикване се изпълнява, когато заявката завърши
- Не блокира извикващата нишка
- Изисква се за архитектури, базирани на събития, GUI приложения или еднонишкови цикли на събития
- Позволява верижно изпълнение на множество операции
- По-сложен поток на управлението

Наборът от тестове на SDK използва изключително `.get()`, но това е подходящо за тестовата среда, където блокирането е приемливо.