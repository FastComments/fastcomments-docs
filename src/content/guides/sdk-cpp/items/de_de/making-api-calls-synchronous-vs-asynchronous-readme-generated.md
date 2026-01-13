Alle API-Methoden in diesem SDK geben `pplx::task<std::shared_ptr<ResponseType>>` aus dem C++ REST SDK zurück. Das gibt Ihnen Flexibilität dabei, wie Sie API-Antworten verarbeiten.

### Synchrone Aufrufe mit `.get()`

Verwenden Sie `.get()`, um den aufrufenden Thread zu blockieren, bis die Anfrage abgeschlossen ist, und das Ergebnis synchron abzurufen:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Rufen Sie .get() auf, um zu blockieren und das Ergebnis synchron zu erhalten
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
).get();  // Blockiert, bis die HTTP-Anfrage abgeschlossen ist

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchrone Aufrufe mit `.then()`

Verwenden Sie `.then()` für nicht-blockierende asynchrone Ausführung mit Rückrufen:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Verwenden Sie .then() für asynchrone, callback-basierte Ausführung
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Dies wird asynchron ausgeführt, wenn die Anfrage abgeschlossen ist
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### Entscheidung zwischen synchronen und asynchronen Aufrufen

Die Wahl hängt von Ihrer Laufzeitumgebung und der Architektur Ihrer Anwendung ab:

**`.get()` (Synchrones Blockieren)**
- Blockiert den aufrufenden Thread, bis die HTTP-Anfrage abgeschlossen ist
- Einfacherer Programmablauf, leichter nachzuvollziehen
- Geeignet für dedizierte Worker-Threads, Batch-Verarbeitung oder Kommandozeilen-Tools
- **Nicht geeignet** für Event-Loops, GUI-Threads oder Single-Thread-Server

**`.then()` (Asynchron, nicht-blockierend)**
- Gibt sofort zurück, der Callback wird ausgeführt, wenn die Anfrage abgeschlossen ist
- Blockiert den aufrufenden Thread nicht
- Erforderlich für ereignisgesteuerte Architekturen, GUI-Anwendungen oder single-threaded Event-Loops
- Ermöglicht das Verketten mehrerer Operationen
- Komplexere Steuerungsabläufe

Die Test-Suite des SDK verwendet ausschließlich `.get()`, was für die Testumgebung angemessen ist, in der Blockieren akzeptabel ist.