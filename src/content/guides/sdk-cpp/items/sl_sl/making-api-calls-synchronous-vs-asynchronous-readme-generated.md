All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Sinhroni klici z `.get()`

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Zahtevani parametri so pozicijski; neobvezni grejo v strukturo možnosti
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Pokličite .get(), da blokirate in pridobite rezultat sinhrono
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokira, dokler HTTP zahteva ne zaključi

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinhroni klici z `.then()`

Use `.then()` for non-blocking asynchronous execution with callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Zahtevani parametri so pozicijski; neobvezni grejo v strukturo možnosti
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Uporabite .then() za asinhrono izvajanje s povratnimi klici
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // To se izvede asinhrono, ko se zahteva zaključi
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvajanje se takoj nadaljuje brez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbira med sinhronim in asinhronim

The choice depends on your runtime environment and application architecture:

**`.get()` (Synchronous blocking)**
- Blokira klicno nit, dokler HTTP zahteva ne zaključi
- Preprostejši potek kode, lažje razumljiv
- Primeren za namenjene delovne niti, paketna obdelovanja ali orodja v ukazni vrstici
- **Ni primerno** za zanke dogodkov, GUI niti ali enovratne strežnike

**`.then()` (Asynchronous non-blocking)**
- Vrne takoj, povratni klic se izvede, ko se zahteva zaključi
- Ne blokira klicne niti
- Potrebno za dogodkovno usmerjene arhitekture, GUI aplikacije ali enovratne zanke dogodkov
- Omogoča zaporedno povezovanje več operacij
- Bolj zapleten potek nadzora

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.