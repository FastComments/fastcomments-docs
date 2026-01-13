Sve API metode u ovom SDK-u vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK-a. Ovo vam daje fleksibilnost u načinu na koji obrađujete API odgovore.

### Sinhroni pozivi pomoću `.get()`

Koristite `.get()` da blokirate pozivajuću nit dok se zahtjev ne završi i sinhrono dobijete rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // page
    boost::none,  // limit
    boost::none,  // skip
    boost::none,  // asTree
    boost::none,  // skipChildren
    boost::none,  // limitChildren
    boost::none,  // maxTreeDepth
    utility::conversions::to_string_t("your-url-id"),  // urlId
    boost::none,  // userId
    boost::none,  // anonUserId
    boost::none,  // contextUserId
    boost::none,  // hashTag
    boost::none,  // parentId
    boost::none   // direction
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinhroni pozivi pomoću `.then()`

Koristite `.then()` za neblokirajuće asinhrono izvršavanje sa povratnim funkcijama:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // This runs asynchronously when the request completes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinhronog i asinhronog

Izbor zavisi od vašeg runtime okruženja i arhitekture aplikacije:

**`.get()` (Synchronous blocking)**
- Blokira pozivajuću nit dok se HTTP zahtjev ne završi
- Jednostavniji tok koda, lakše za razumijevanje
- Pogodno za namjenske radne niti, batch obradu ili alate iz komandne linije
- **Nije pogodno** za petlje događaja, GUI niti ili servere sa jednom niti

**`.then()` (Asynchronous non-blocking)**
- Vraća se odmah, povratna funkcija se izvršava kad se zahtjev završi
- Ne blokira pozivajuću nit
- Potrebno za arhitekture vođene događajima, GUI aplikacije ili petlje događaja u jednoj niti
- Omogućava ulančavanje više operacija
- Složeniji tok kontrole

Testni skup SDK-a koristi `.get()` isključivo, ali to je prikladno za testno okruženje gdje je blokiranje prihvatljivo.