All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Sinkrone pozive s `.get()`

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Potrebni parametri su položeni; opcionalni idu u strukturu opcija
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Pozovite .get() da blokirate i dohvatite rezultat sinkrono
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokira dok HTTP zahtjev ne završi

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinkroni pozivi s `.then()`

Use `.then()` for non-blocking asynchronous execution with callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Potrebni parametri su položeni; opcionalni idu u strukturu opcija
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Koristite .then() za asinkrono izvođenje temeljeno na povratnim pozivima
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ovo se izvršava asinkrono kada zahtjev završi
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršavanje nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Odabir između sinkronog i asinkronog

The choice depends on your runtime environment and application architecture:

**`.get()` (Sinkrono blokiranje)**
- Blokira pozivni thread dok HTTP zahtjev ne zavši
- Jednostavniji tijek koda, lakše razumljivo
- Pogodno za namjenske radne threadove, batch obradu ili alate za naredbeni redak
- **Nije pogodno** za petlje događaja, GUI threadove ili jednonitne servere

**`.then()` (Asinkrono neblokiranje)**
- Vraća se odmah, povratni poziv se izvršava kada zahtjev završi
- Ne blokira pozivni thread
- Potrebno za arhitekture temeljene na događajima, GUI aplikacije ili jednonitne petlje događaja
- Omogućuje lančanje više operacija
- Kompleksniji kontrolni tijek

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.