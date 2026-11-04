Vse metode API v tem SDK-ju vračajo `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK. To vam daje prilagodljivost pri obdelavi odgovorov API-ja.

### Sinhroni klici z `.get()`

Uporabite `.get()`, da blokirate klicno nit, dokler zahteva ni zaključena, in sinhrono pridobite rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Obvezni parametri so pozicijski; neobvezni gredo v options strukturo
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Pokličite .get(), da blokirate in sinhrono pridobite rezultat
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokira, dokler HTTP zahteva ni zaključena

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinhroni klici z `.then()`

Uporabite `.then()` za neblokirajoče asinhrono izvajanje s povratnimi klici:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Obvezni parametri so pozicijski; neobvezni gredo v options strukturo
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Uporabite .then() za asinhrono izvajanje s povratnimi klici
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // To se izvede asinhrono, ko je zahteva zaključena
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvajanje se nadaljuje takoj, brez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbira med sinhronim in asinhronim izvajanjem

Izbira je odvisna od vašega izvedbenega okolja in arhitekture aplikacije:

**`.get()` (sinhrono blokiranje)**
- Blokira klicno nit, dokler HTTP zahteva ni zaključena
- Preprostejši potek kode, lažje za razumevanje
- Primerno za namenske delovne niti, paketno obdelavo ali orodja ukazne vrstice
- **Ni primerno** za dogodkovne zanke, GUI niti ali enonitne strežnike

**`.then()` (asinhrono, neblokirajoče)**
- Vrne se takoj, povratni klic se izvede, ko je zahteva zaključena
- Ne blokira klicne niti
- Obvezno za arhitekture na osnovi dogodkov, GUI aplikacije ali enonitne dogodkovne zanke
- Omogoča veriženje več operacij
- Bolj zapleten nadzor poteka

Testna zbirka SDK-ja izključno uporablja `.get()`, kar je primerno za testno okolje, kjer je blokiranje sprejemljivo.