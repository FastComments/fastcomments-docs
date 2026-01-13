Sve API metode u ovom SDK-u vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK-a. To vam daje fleksibilnost u načinu rukovanja odgovorima API-ja.

### Sinkroni pozivi s `.get()`

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Pozovite .get() da blokira pozivajuću nit i sinhrono dobije rezultat
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

// Koristite .then() za asinkrono izvršavanje temeljeno na povratnim pozivima
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ovo se izvršava asinkrono kada zahtjev završi
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršavanje se nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbor između sinkronog i asinkronog

Izbor ovisi o vašem runtime okruženju i arhitekturi aplikacije:

**`.get()` (Synchronous blocking)**
- Blokira pozivajuću nit dok HTTP zahtjev ne završi
- Jednostavniji tijek koda, lakše za razumjeti
- Pogodno za namjenske radne niti, batch obradu ili alate naredbenog retka
- **Nije pogodno** za petlje događaja, GUI niti ili poslužitelje s jednom niti

**`.then()` (Asynchronous non-blocking)**
- Vraća odmah, povratni poziv se izvršava kada zahtjev završi
- Ne blokira pozivajuću nit
- Potrebno za arhitekture vođene događajima, GUI aplikacije ili petlje događaja s jednom niti
- Omogućava lančanje više operacija
- Složeniji tijek kontrole

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.