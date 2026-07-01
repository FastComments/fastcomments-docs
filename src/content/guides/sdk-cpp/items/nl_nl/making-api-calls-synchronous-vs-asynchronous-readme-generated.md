All API‑methoden in deze SDK retourneren `pplx::task<std::shared_ptr<ResponseType>>` vanuit de C++ REST SDK. Dit geeft je flexibiliteit in hoe je API‑reacties afhandelt.

### Synchrone Aanroepen met `.get()`

Gebruik `.get()` om de aanroepende thread te blokkeren totdat het verzoek is voltooid en om het resultaat synchroon op te halen:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Vereiste parameters zijn positioneel; optionele gaan in de optiestructuur
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Roep .get() aan om te blokkeren en het resultaat synchroon op te halen
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blokkeert totdat het HTTP-verzoek is voltooid

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchrone Aanroepen met `.then()`

Gebruik `.then()` voor niet-blokkerende asynchrone uitvoering met callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Vereiste parameters zijn positioneel; optionele gaan in de optiestructuur
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Gebruik .then() voor asynchrone callback‑gebaseerde uitvoering
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Dit wordt asynchroon uitgevoerd wanneer het verzoek is voltooid
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Uitvoering gaat onmiddellijk verder zonder te blokkeren
std::cout << "Request sent, continuing..." << std::endl;
```

### Keuze tussen synchrone en asynchrone

De keuze hangt af van je runtime‑omgeving en applicatie‑architectuur:

**`.get()` (Synchrone blokkering)**
- Blokkeert de aanroepende thread totdat het HTTP‑verzoek is voltooid
- Eenvoudigere codeflow, makkelijker te begrijpen
- Geschikt voor toegewijde werkerthreads, batchverwerking of opdrachtregeltools
- **Niet geschikt** voor event loops, GUI‑threads of single‑threaded servers

**`.then()` (Asynchrone niet‑blokkering)**
- Retourneert onmiddellijk, callback wordt uitgevoerd wanneer het verzoek is voltooid
- Blokkeert de aanroepende thread niet
- Vereist voor event‑gedreven architecturen, GUI‑toepassingen of single‑threaded event loops
- Staat chaining van meerdere bewerkingen toe
- Complexere controleflow

De testreeks van de SDK gebruikt uitsluitend `.get()`, maar dit is passend voor de testomgeving waar blokkeren acceptabel is.