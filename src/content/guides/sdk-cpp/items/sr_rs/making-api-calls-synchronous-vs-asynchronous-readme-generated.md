Све API методе у овом SDK-у враћају `pplx::task<std::shared_ptr<ResponseType>>` из C++ REST SDK-а. Ово вам даје флексибилност у начину на који обрађујете одговоре API-ја.

### Синхрони позиви помоћу `.get()`

Користите `.get()` да бисте блокирали позивачки нит све док захтев не заврши и добили резултат синхроно:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Позовите .get() да бисте блокирали и добили резултат синхроно
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
).get();  // Блокира док HTTP захтев не буде завршен

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхрони позиви помоћу `.then()`

Користите `.then()` за не-блокирајуће асинхроно извршавање са повратним позивима (callbacks):

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Користите .then() за асинхроно извршавање засновано на повратним позивима
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ово се извршава асинхроно када захтев буде завршен
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Извршавање се наставља одмах без блокирања
std::cout << "Request sent, continuing..." << std::endl;
```

### Избор између синхроног и асинхроног

Избор зависи од вашег окружења за извршавање и архитектуре апликације:

**`.get()` (Синхроно, блокирајуће)**
- Блокира позивачки нит док HTTP захтев не буде завршен
- Једноставнији ток кода, лакше за разумевање
- Погодно за посвећене радничке нити, пакетну обраду или алате командне линије
- **Није погодно** за петље догађаја, GUI нити или једнонитне сервере

**`.then()` (Асинхроно, не-блокирајуће)**
- Враћа се одмах, повратни позив се извршава када захтев буде завршен
- Не блокира позивачки нит
- Потребно за архитектуре засноване на догађајима, GUI апликације или једнонитне петље догађаја
- Омогућава ланчање више операција
- Сложенији ток контроле

Тестни скуп SDK-а користи искључиво `.get()`, али то је прикладно за тестно окружење где је блокирање прихватљиво.