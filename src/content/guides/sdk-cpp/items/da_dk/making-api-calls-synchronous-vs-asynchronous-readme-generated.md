Alle API-metoder i denne SDK returnerer `pplx::task<std::shared_ptr<ResponseType>>` fra C++ REST SDK. Dette giver dig fleksibilitet i, hvordan du håndterer API-svar.

### Synkrone kald med `.get()`

Brug `.get()` til at blokere den kaldende tråd, indtil anmodningen er fuldført, og hent resultatet synkront:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Påkrævede parametre er positionelle; valgfrie placeres i options-strukturen
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Kald .get() for at blokere og hente resultatet synkront
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokerer, indtil HTTP-anmodningen er fuldført

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynkrone kald med `.then()`

Brug `.then()` til ikke-blokerende asynkron udførelse med callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Påkrævede parametre er positionelle; valgfrie placeres i options-strukturen
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Brug .then() til asynkron callback-baseret udførelse
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Dette køres asynkront, når anmodningen er fuldført
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Udførelsen fortsætter straks uden at blokere
std::cout << "Request sent, continuing..." << std::endl;
```

### Valg mellem synkron og asynkron

Valget afhænger af dit runtime-miljø og din applikationsarkitektur:

**`.get()` (Synkron blokering)**
- Blokerer den kaldende tråd, indtil HTTP-anmodningen er fuldført
- Simpler kodeflow, lettere at ræsonnere om
- Velegnet til dedikerede worker-tråde, batchbehandling eller kommandolinjeværktøjer
- **Ikke velegnet** til event loops, GUI-tråde eller single-threaded servere

**`.then()` (Asynkron ikke-blokerende)**
- Returnerer straks, callback udføres, når anmodningen er fuldført
- Blokerer ikke den kaldende tråd
- Påkrævet til begivenhedsdrevne arkitekturer, GUI-applikationer eller single-threaded event loops
- Muliggør kædning af flere operationer
- Mere komplekst kontrolflow

SDK'ens testpakke bruger udelukkende `.get()`, men det er passende i testmiljøet, hvor blokering er acceptabelt.