All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Sinhroni pozivi sa `.get()`

Koristite `.get()` da blokirate pozivni thread dok se zahtjev ne završi i da sinhrono dobijete rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Potrebni parametri su pozicioni; opcionalni idu u strukturu opcija
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Pozovite .get() da blokirate i dobijete rezultat sinhrono
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokira dok HTTP zahtjev ne završi

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinhroni pozivi sa `.then()`

Koristite `.then()` za neblokirajuću asinhronu izvršavanje s povratnim pozivima (callback-ovima):

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Potrebni parametri su pozicioni; opcionalni idu u strukturu opcija
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Koristite .then() za asinhrono izvršavanje zasnovano na povratnim pozivima
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ovo se izvršava asinhrono kada se zahtjev završi
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršenje nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinhronog i asinhronog

Izbor zavisi od vašeg okruženja za izvršavanje i arhitekture aplikacije:

**`.get()` (Sinhrono blokiranje)**
- Blokira pozivni thread dok HTTP zahtjev ne završi
- Jednostavniji tok koda, lakše razumljiv
- Pogodno za dedikovane radne thread-ove, batch obradu ili alate na komandnoj liniji
- **Nije pogodno** za petlje događaja, GUI thread-ove ili jednonitne servere

**`.then()` (Asinhrono neblokirajuće)**
- Vraća se odmah, povratni poziv se izvršava kada zahtjev završi
- Ne blokira pozivni thread
- Potrebno za arhitekture zasnovane na događajima, GUI aplikacije ili jednonitne petlje događaja
- Omogućava lančanje više operacija
- Kompleksniji tok kontrole

Testni skup SDK-a koristi `.get()` ekskluzivno, ali to je prikladno za testno okruženje gdje je blokiranje prihvatljivo.