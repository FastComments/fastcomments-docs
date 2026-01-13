Svi API metode u ovom SDK vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK-a. Ovo vam daje fleksibilnost u načinu na koji obrađujete odgovore API-ja.

### Sinhroni pozivi sa `.get()`

Koristite `.get()` da blokirate pozivajući thread dok zahtev ne bude završen i sinhrono dobijete rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Pozovite .get() da blokirate i sinhrono dobijete rezultat
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // stranica
    boost::none,  // limit
    boost::none,  // preskok
    boost::none,  // kaoStablo
    boost::none,  // preskociDecu
    boost::none,  // limitDeca
    boost::none,  // maksDubinaStabla
    utility::conversions::to_string_t("your-url-id"),  // idUrl-a
    boost::none,  // idKorisnika
    boost::none,  // idAnonKorisnika
    boost::none,  // idKontekstKorisnika
    boost::none,  // hašTag
    boost::none,  // idRoditelja
    boost::none   // smer
).get();  // Blokira dok HTTP zahtev ne bude završen

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinhroni pozivi sa `.then()`

Koristite `.then()` za neblokirajuće asinhrono izvršavanje sa callback-ovima:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Koristite .then() za asinhrono izvršavanje zasnovano na callback-ovima
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ovo se izvršava asinhrono kada zahtev bude završen
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršavanje se nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinhronog i asinhronog

Izbor zavisi od vašeg runtime okruženja i arhitekture aplikacije:

**`.get()` (Sinhrono — blokira)**
- Blokira pozivajući thread dok HTTP zahtev ne bude završen
- Jednostavniji tok koda, lakše za razumevanje
- Pogodno za posvećene radne niti, batch obradu ili komandnu liniju
- **Nije pogodno** za event loop-ove, GUI thread-ove ili jednothread servere

**`.then()` (Asinhrono — neblokirajuće)**
- Vraća se odmah, callback se izvršava kada zahtev bude završen
- Ne blokira pozivajući thread
- Potrebno za event-driven arhitekture, GUI aplikacije ili jednothread event loop-ove
- Omogućava lančanje više operacija
- Složeniji tok kontrole

Test-suite SDK-a koristi isključivo `.get()`, ali to je odgovarajuće za test okruženje gde je blokiranje prihvatljivo.