Vse API metode v tem SDK vračajo `pplx::task<std::shared_ptr<ResponseType>>` iz C++ REST SDK. To vam daje fleksibilnost pri tem, kako upravljate odzive API-ja.

### Sinhroni klici z `.get()`

Uporabite `.get()`, da zaklenete klicno nit, dokler se zahteva ne zaključi, in sinhrono pridobite rezultat:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Pokliči .get(), da blokiraš nit in sinhrono pridobiš rezultat
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // stran
    boost::none,  // omejitev
    boost::none,  // preskoči
    boost::none,  // kot drevo
    boost::none,  // preskoči otroke
    boost::none,  // omejitev otrok
    boost::none,  // največja globina drevesa
    utility::conversions::to_string_t("your-url-id"),  // id URL-ja
    boost::none,  // id uporabnika
    boost::none,  // id anonimnega uporabnika
    boost::none,  // id uporabnika konteksta
    boost::none,  // hashTag
    boost::none,  // id nadrejenega
    boost::none   // smer
).get();  // Blokira, dokler se HTTP zahteva ne zaključi

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

// Uporabi .then() za asinhrono izvajanje z povratnimi klici
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // To se izvede asinhrono, ko je zahteva zaključena
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Izvedba se nadaljuje takoj, brez blokiranja
std::cout << "Request sent, continuing..." << std::endl;
```

### Izbira med sinhronimi in asinhronimi klici

Izbira je odvisna od vašega runtime okolja in arhitekture aplikacije:

**`.get()` (Sinhrono, blokira)**
- Blokira klicno nit, dokler se HTTP zahteva ne zaključi
- Preprostejši potek kode, lažje za razumevanje
- Primeren za namenska delovna vlakna, obdelavo paketov ali ukazno-vrstične pripomočke
- **Ni primeren** za zanke dogodkov, GUI niti ali enovlaknene strežnike

**`.then()` (Asinhrono, neblokirajoče)**
- Vrne takoj, povratni klic se izvede, ko je zahteva zaključena
- Ne blokira klicne niti
- Potreben za arhitekture, ki temeljijo na dogodkih, GUI aplikacije ali enovlaknene zanke dogodkov
- Omogoča zaporedno vezavo več operacij
- Kompleksnejši potek nadzora

Testni nabor SDK uporablja izključno `.get()`, kar je primerno za testno okolje, kjer je blokiranje sprejemljivo.