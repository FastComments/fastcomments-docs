Alle API-metoder i dette SDK returnerer `pplx::task<std::shared_ptr<ResponseType>>` fra C++ REST SDK. Det giver dig fleksibilitet i, hvordan du håndterer API-svar.

### Synkrone kald med `.get()`

Brug `.get()` til at blokere den kaldende tråd, indtil anmodningen er færdig, og hent resultatet synkront:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Kald .get() for at blokere og få resultatet synkront
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
).get();  // Blokerer indtil HTTP-anmodningen er fuldført

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

// Brug .then() til asynkron callback-baseret udførelse
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Dette kører asynkront, når anmodningen afsluttes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Udførelsen fortsætter straks uden blokering
std::cout << "Request sent, continuing..." << std::endl;
```

### Valg mellem synkront og asynkront

Valget afhænger af dit runtime-miljø og din applikationsarkitektur:

**`.get()` (Synkront, blokerende)**
- Blokerer den kaldende tråd, indtil HTTP-anmodningen er fuldført
- Enklere kodeflow, lettere at forstå
- Egnet til dedikerede worker-tråde, batchbehandling eller kommandolinjeværktøjer
- **Ikke egnet** til hændelsesløkker, GUI-tråde eller enkelttrådede servere

**`.then()` (Asynkront, ikke-blokerende)**
- Returnerer med det samme; callback'en udføres, når anmodningen er fuldført
- Blokerer ikke den kaldende tråd
- Påkrævet for hændelsesdrevne arkitekturer, GUI-applikationer eller enkelttrådede hændelsesløkker
- Tillader kædning af flere operationer
- Mere kompleks kontrolflow

SDK'ets testsuite bruger udelukkende `.get()`, men det er passende i testmiljøet, hvor blokering er acceptabel.