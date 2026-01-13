Alle API-methoden in deze SDK retourneren `pplx::task<std::shared_ptr<ResponseType>>` van de C++ REST SDK. Dit geeft u flexibiliteit in hoe u API-antwoorden afhandelt.

### Synchrone oproepen met `.get()`

Gebruik `.get()` om de aanroepende thread te blokkeren totdat het verzoek is voltooid en het resultaat synchroon op te halen:

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

### Asynchrone oproepen met `.then()`

Gebruik `.then()` voor niet-blokkerende asynchrone uitvoering met callbacks:

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

### Kiezen tussen synchrone en asynchrone

De keuze hangt af van uw runtime-omgeving en applicatiearchitectuur:

**`.get()` (Synchroon, blokkerend)**
- Blokkeert de aanroepende thread totdat het HTTP-verzoek is voltooid
- Eenvoudigere codeflow, makkelijker te begrijpen
- Geschikt voor toegewijde worker-threads, batchverwerking of commandoregelprogramma's
- **Niet geschikt** voor event loops, GUI-threads of single-threaded servers

**`.then()` (Asynchroon, niet-blokkerend)**
- Keert onmiddellijk terug, de callback wordt uitgevoerd wanneer het verzoek is voltooid
- Blokkeert de aanroepende thread niet
- Vereist voor gebeurtenisgestuurde architecturen, GUI-applicaties of single-threaded event loops
- Maakt het mogelijk meerdere bewerkingen aan elkaar te schakelen
- Complexere controleflow

De testsuite van de SDK gebruikt uitsluitend `.get()`, maar dit is geschikt voor de testomgeving waar blokkering acceptabel is.