Tutti i metodi API in questo SDK restituiscono `pplx::task<std::shared_ptr<ResponseType>>` dal C++ REST SDK. Questo ti dà flessibilità nel modo in cui gestisci le risposte API.

### Chiamate sincrone con `.get()`

Usa `.get()` per bloccare il thread chiamante finché la richiesta non è completata e recuperare il risultato in modo sincrono:

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

### Chiamate asincrone con `.then()`

Usa `.then()` per esecuzione asincrona non bloccante con callback:

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

### Scegliere tra sincrono e asincrono

La scelta dipende dal tuo ambiente di runtime e dall'architettura dell'applicazione:

**`.get()` (Blocco sincrono)**
- Blocca il thread chiamante finché la richiesta HTTP non è completata
- Flusso di codice più semplice, più facile da comprendere
- Adatto a thread di lavoro dedicati, elaborazione batch o strumenti da riga di comando
- **Non adatto** a loop di eventi, thread GUI o server monothread

**`.then()` (Asincrono non bloccante)**
- Ritorna immediatamente, il callback viene eseguito quando la richiesta è completata
- Non blocca il thread chiamante
- Necessario per architetture guidate dagli eventi, applicazioni GUI o loop di eventi monothread
- Consente il concatenamento di più operazioni
- Flusso di controllo più complesso

La suite di test dell'SDK utilizza `.get()` esclusivamente, ma ciò è appropriato per l'ambiente di test dove il blocco è accettabile.