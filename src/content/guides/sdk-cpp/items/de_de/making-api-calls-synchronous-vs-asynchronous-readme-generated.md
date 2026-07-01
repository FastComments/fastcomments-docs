Alle API-Methoden in diesem SDK geben `pplx::task<std::shared_ptr<ResponseType>>` aus dem C++ REST SDK zurück. Das gibt Ihnen Flexibilität im Umgang mit API‑Antworten.

### Synchrone Aufrufe mit `.get()`

Verwenden Sie `.get()`, um den aufrufenden Thread zu blockieren, bis die Anfrage abgeschlossen ist, und das Ergebnis synchron abzurufen:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchrone Aufrufe mit `.then()`

Verwenden Sie `.then()` für nicht‑blockierende asynchrone Ausführung mit Callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // This runs asynchronously when the request completes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### Auswahl zwischen synchron und asynchron

Die Wahl hängt von Ihrer Laufzeitumgebung und Anwendungsarchitektur ab:

**`.get()` (Synchronous blocking)**
- Blockiert den aufrufenden Thread, bis die HTTP‑Anfrage abgeschlossen ist
- Einfacherer Codefluss, leichter nachvollziehbar
- Geeignet für dedizierte Worker‑Threads, Batch‑Verarbeitung oder Kommandozeilen‑Tools
- **Nicht geeignet** für Ereignisschleifen, GUI‑Threads oder ein‑threadige Server

**`.then()` (Asynchronous non-blocking)**
- Gibt sofort zurück, der Callback wird ausgeführt, wenn die Anfrage abgeschlossen ist
- Blockiert den aufrufenden Thread nicht
- Erforderlich für ereignisgesteuerte Architekturen, GUI‑Anwendungen oder ein‑threadige Ereignisschleifen
- Ermöglicht das Verketten mehrerer Operationen
- Komplexerer Kontrollfluss

Die Testsuite des SDK verwendet ausschließlich `.get()`, was jedoch für die Testumgebung, in der Blockieren akzeptabel ist, passend ist.