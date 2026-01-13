Sve API metode u ovom SDK-u vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK-a. Ovo vam daje fleksibilnost u načinu rukovanja odgovorima API-ja.

### Sinhroni pozivi sa `.get()`

Koristite `.get()` da blokirate pozivajuću nit dok zahtjev ne bude završen i da sinhrono dobijete rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Pozovite .get() da blokirate i dobijete rezultat sinhrono
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // page (stranica)
    boost::none,  // limit (limit)
    boost::none,  // skip (preskoči)
    boost::none,  // asTree (kao stablo)
    boost::none,  // skipChildren (preskoči potomke)
    boost::none,  // limitChildren (ograniči potomke)
    boost::none,  // maxTreeDepth (maksimalna dubina stabla)
    utility::conversions::to_string_t("your-url-id"),  // urlId (urlId)
    boost::none,  // userId (userId, ID korisnika)
    boost::none,  // anonUserId (anonUserId, ID anonimnog korisnika)
    boost::none,  // contextUserId (contextUserId, ID kontekstnog korisnika)
    boost::none,  // hashTag (hashTag)
    boost::none,  // parentId (parentId, ID roditelja)
    boost::none   // direction (direction)
).get();  // Blokira dok se HTTP zahtjev ne završi

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
    // Ovo se izvršava asinhrono kada se zahtjev završi
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršavanje se nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinhronog i asinhronog

Izbor zavisi od vašeg runtime okruženja i arhitekture aplikacije:

**`.get()` (Sinhrono blokiranje)**
- Blokira pozivajuću nit dok se HTTP zahtjev ne završi
- Jednostavniji tok koda, lakše za razumijevanje
- Pogodno za namjenske radne niti, batch obradu ili komandno-linijske alate
- **Nije pogodno** za petlje događaja, GUI niti ili servere sa jednom niti

**`.then()` (Asinhrono, neblokirajuće)**
- Vraća odmah, callback se izvršava kada se zahtjev završi
- Ne blokira pozivajuću nit
- Neophodno za arhitekture vođene događajima, GUI aplikacije ili petlje događaja sa jednom niti
- Omogućava lančanje više operacija
- Složeniji tok kontrole

Testni paket SDK-a koristi isključivo `.get()`, što je prikladno za testno okruženje gdje je blokiranje prihvatljivo.