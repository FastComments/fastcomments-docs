Bu SDK'daki tüm API yöntemleri C++ REST SDK'dan `pplx::task<std::shared_ptr<ResponseType>>` döndürür. Bu, API yanıtlarını nasıl ele alacağınıza ilişkin esneklik sağlar.

### Eşzamanlı Çağrılar `.get()` ile

İstek tamamlanana kadar çağıran iş parçacığını engellemek ve sonucu eşzamanlı olarak almak için `.get()` kullanın:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// İsteğin tamamlanmasına kadar engellemek ve sonucu eşzamanlı almak için .get() çağırın
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // sayfa
    boost::none,  // limit
    boost::none,  // atla
    boost::none,  // ağaç olarak
    boost::none,  // çocukları atla
    boost::none,  // çocuk limiti
    boost::none,  // maksimum ağaç derinliği
    utility::conversions::to_string_t("your-url-id"),  // urlId
    boost::none,  // userId
    boost::none,  // anonUserId
    boost::none,  // contextUserId
    boost::none,  // hashTag
    boost::none,  // parentId
    boost::none   // direction
).get();  // HTTP isteği tamamlanana kadar engeller

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asenkron Çağrılar `.then()` ile

Geri arama tabanlı, bloklamayan asenkron yürütme için `.then()` kullanın:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Geri arama tabanlı asenkron yürütme için .then() kullanın
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // İstek tamamlandığında bu asenkron olarak çalışır
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Bloklamadan yürütme hemen devam eder
std::cout << "Request sent, continuing..." << std::endl;
```

### Eşzamanlı ile Asenkron Arasında Seçim Yapma

Seçim, çalışma zamanı ortamınıza ve uygulama mimarinize bağlıdır:

**`.get()` (Eşzamanlı bloklama)**
- HTTP isteği tamamlanana kadar çağıran iş parçacığını engeller
- Daha basit kod akışı, anlaşılması daha kolay
- Özel işçi iş parçacıkları, toplu işleme veya komut satırı araçları için uygundur
- Olay döngüleri, GUI iş parçacıkları veya tek iş parçacıklı sunucular için **uygun değildir**

**`.then()` (Asenkron bloklamayan)**
- Hemen döner, istek tamamlandığında geri arama yürütülür
- Çağıran iş parçacığını engellemez
- Olay tabanlı mimariler, GUI uygulamaları veya tek iş parçacıklı olay döngüleri için gereklidir
- Birden çok işlemi zincirleme olanağı sağlar
- Kontrol akışı daha karmaşıktır

SDK'nın test paketi yalnızca `.get()` kullanır, ancak bu, bloklamanın kabul edilebilir olduğu test ortamı için uygundur.