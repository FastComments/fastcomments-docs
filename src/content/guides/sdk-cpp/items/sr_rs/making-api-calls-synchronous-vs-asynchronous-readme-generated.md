Svi API metodi u ovom SDK‑u vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK‑a. Ovo vam daje fleksibilnost u načinu na koji obrađujete API odgovore.

### Sinhroni pozivi sa `.get()`

Koristite `.get()` da blokirate nit pozivaoca dok zahtev ne bude završen i da sinhrono preuzmete rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Obavezni parametri su pozicioni; opciona idu u strukturu opcija
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Pozovite .get() da blokirate i sinhrono dobijete rezultat
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokira dok HTTP zahtev ne bude završen

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinhroni pozivi sa `.then()`

Koristite `.then()` za neblokirajuće asinhrono izvršavanje sa povratnim pozivima:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Obavezni parametri su pozicioni; opciona idu u strukturu opcija
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Koristite .then() za asinhrono izvršavanje zasnovano na povratnim pozivima
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ovo se izvršava asinhrono kada zahtev bude završen
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršavanje nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinhronog i asinhronog pristupa

Izbor zavisi od vašeg okruženja za izvršavanje i arhitekture aplikacije:

**`.get()` (Sinhrono blokiranje)**
- Blokira nit pozivaoca dok HTTP zahtev ne bude završen
- Jednostavniji tok koda, lakše za razumevanje
- Pogodno za posvećene radne niti, batch obradu ili alate komandne linije
- **Neodgovarajuće** za petlje događaja, GUI niti ili jednonitne servere

**`.then()` (Asinhrono neblokiranje)**
- Vraća se odmah, povratni poziv se izvršava kada zahtev bude završen
- Ne blokira nit pozivaoca
- Neophodno za event‑driven arhitekture, GUI aplikacije ili jednonitne petlje događaja
- Omogućava ulančavanje višestrukih operacija
- Složeniji tok kontrole

Test paket SDK‑a koristi isključivo `.get()`, ali to je prikladno za test okruženje u kome je blokiranje prihvatljivo.