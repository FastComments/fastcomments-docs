Bu SDK'daki tüm API metotları, C++ REST SDK'dan `pplx::task<std::shared_ptr<ResponseType>>` döndürür. Bu, API yanıtlarını işleme konusunda size esneklik sağlar.

### `.get()` ile Senkron Çağrılar

Çağıran iş parçacığını (thread) istek tamamlanana kadar bloke etmek ve sonucu senkron olarak almak için `.get()` kullanın:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Zorunlu parametreler sıralıdır; isteğe bağlı olanlar options yapısına girer
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Bloke etmek ve sonucu senkron olarak almak için .get() çağırın
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // HTTP isteği tamamlanana kadar bloke eder

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()` ile Asenkron Çağrılar

Geri çağırmalarla (callback) bloke etmeyen asenkron yürütme için `.then()` kullanın:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Zorunlu parametreler sıralıdır; isteğe bağlı olanlar options yapısına girer
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Geri çağırma tabanlı asenkron yürütme için .then() kullanın
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Bu, istek tamamlandığında asenkron olarak çalışır
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Yürütme, bloke etmeden hemen devam eder
std::cout << "Request sent, continuing..." << std::endl;
```

### Senkron ve Asenkron Arasında Seçim Yapma

Seçim, çalışma zamanı ortamınıza ve uygulama mimarinize bağlıdır:

**`.get()` (Senkron bloke etme)**
- HTTP isteği tamamlanana kadar çağıran iş parçacığını bloke eder
- Daha basit kod akışı, akıl yürütmesi daha kolay
- Ayrılmış işçi (worker) iş parçacıkları, toplu işleme veya komut satırı araçları için uygundur
- Olay döngüleri, GUI iş parçacıkları veya tek iş parçacıklı sunucular için **uygun değildir**

**`.then()` (Asenkron, bloke etmeyen)**
- Hemen döner, geri çağırma istek tamamlandığında çalışır
- Çağıran iş parçacığını bloke etmez
- Olay güdümlü mimariler, GUI uygulamaları veya tek iş parçacıklı olay döngüleri için gereklidir
- Birden fazla işlemin zincirlenmesine olanak tanır
- Daha karmaşık kontrol akışı

SDK'nın test paketi yalnızca `.get()` kullanır, ancak bu, bloke etmenin kabul edilebilir olduğu test ortamı için uygundur.