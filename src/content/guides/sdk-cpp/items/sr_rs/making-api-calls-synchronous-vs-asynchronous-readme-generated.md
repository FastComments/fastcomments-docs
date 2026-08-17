Све API методе у овом SDK-у враћају `pplx::task<std::shared_ptr<ResponseType>>` из C++ REST SDK-а. Ово вам пружа флексибилност у начину на који руковате API одговорима.

### Синхроне позиве са `.get()`

Користите `.get()` да блокирате позивни нит док захтев не заврши и да синхроно добијете резултат:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Потребни параметри су позиционални; опционо су у структури options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Позовите .get() да блокирате и добијете резултат синхроно
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Блокира док HTTP захтев не заврши

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Асинхроне позиве са `.then()`

Користите `.then()` за не-блокирајуће асинхроно извршавање са повратним позивима:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Потребни параметри су позиционални; опционо су у структури options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Користите .then() за асинхрону извршавање засновано на повратним позивима
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ово се извршава асинхроно када се захтев заврши
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Извршавање наставља одмах без блокирања
std::cout << "Request sent, continuing..." << std::endl;
```

### Одабир између синхроног и асинхроног

Избор зависи од вашег окружења за извршавање и архитектуре апликације:

**`.get()` (Синхроно блокирање)**
- Блокира позивни нит док HTTP захтев не заврши
- Једноставнији ток кода, лакше за разумевање
- Погодно за посвећене радне нити, пакетну обраду или алате командне линије
- **Није погодно** за петље догађаја, GUI нити или једнонитне сервере

**`.then()` (Асинхроно не-блокирање)**
- Враћа се одмах, повратни позив се извршава када се захтев заврши
- Не блокира позивни нит
- Потребно за архитектуре засноване на догађајима, GUI апликације или једнонитне петље догађаја
- Омогућава ланцирање више операција
- Сложенији ток контроле

Тестни скуп SDK‑а користи искључиво `.get()`, али ово је прикладно за тестно окружење где је блокирање прихватљиво.