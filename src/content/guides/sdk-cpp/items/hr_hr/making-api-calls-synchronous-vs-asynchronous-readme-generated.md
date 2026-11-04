Sve API metode u ovom SDK-u vraćaju `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK-a. To vam daje fleksibilnost u načinu rukovanja API odgovorima.

### Sinkroni pozivi s `.get()`

Koristite `.get()` za blokiranje pozivajuće dretve dok se zahtjev ne završi i za sinkrono dohvaćanje rezultata:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Obavezni parametri su pozicijski; neobavezni idu u options strukturu
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Pozovite .get() za blokiranje i sinkrono dohvaćanje rezultata
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokira dok se HTTP zahtjev ne završi

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asinkroni pozivi s `.then()`

Koristite `.then()` za neblokirajuće asinkrono izvršavanje s callbackovima:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Obavezni parametri su pozicijski; neobavezni idu u options strukturu
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Koristite .then() za asinkrono izvršavanje temeljeno na callbackovima
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ovo se izvršava asinkrono kada se zahtjev završi
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvršavanje se nastavlja odmah bez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Odabir između sinkronog i asinkronog pristupa

Odabir ovisi o vašem runtime okruženju i arhitekturi aplikacije:

**`.get()` (Sinkrono blokiranje)**
- Blokira pozivajuću dretvu dok se HTTP zahtjev ne završi
- Jednostavniji tijek koda, lakše se razmišlja o njemu
- Pogodno za namjenske radne dretve, batch obradu ili alate naredbenog retka
- **Nije pogodno** za event loope, GUI dretve ili jednodretvene poslužitelje

**`.then()` (Asinkrono neblokirajuće)**
- Vraća se odmah, callback se izvršava kada se zahtjev završi
- Ne blokira pozivajuću dretvu
- Obavezno za arhitekture vođene događajima, GUI aplikacije ili jednodretvene event loope
- Omogućuje ulančavanje više operacija
- Složeniji tijek upravljanja

Testni paket SDK-a koristi isključivo `.get()`, ali to je prikladno za testno okruženje u kojem je blokiranje prihvatljivo.