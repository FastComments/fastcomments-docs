Sve API metode u ovom SDK‑u vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK‑a. Ovo vam pruža fleksibilnost u načinu na koji obrađujete API odgovore.

### Sinhroni pozivi sa `.get()`

Koristite `.get()` da blokirate nit koja poziva dok se zahtev ne završi i da sinhrono dobijete rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinkroni pozivi sa `.then()`

Koristite `.then()` za neblokirajuću asinkronu izvršavanje sa povratnim pozivima (callback-ovima):

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // This runs asynchronously when the request completes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinhronog i asinkronog

Izbor zavisi od vašeg runtime okruženja i arhitekture aplikacije:

**`.get()` (Sinhrono blokiranje)**
- Blokira nit koja poziva dok HTTP zahtev ne bude završen
- Jednostavniji tok koda, lakše je razumeti
- Pogodno za posvećene radne niti, obradu u batch režimu ili komandno‑linijske alate
- **Nije pogodno** za petlje događaja, GUI niti ili jednonitne servere

**`.then()` (Asinkrono neblokirajuće)**
- Vraća se odmah, povratni poziv se izvršava kada zahtev bude završen
- Ne blokira nit koja poziva
- Potrebno za arhitekture bazirane na događajima, GUI aplikacije ili jednonitne petlje događaja
- Omogućava lančanje više operacija
- Kompleksniji tok kontrole

Testni skup SDK‑a koristi isključivo `.get()`, ali to je odgovarajuće za testno okruženje gde je blokiranje prihvatljivo.