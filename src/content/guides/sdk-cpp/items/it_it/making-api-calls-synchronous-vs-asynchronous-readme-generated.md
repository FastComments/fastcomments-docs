Tutti i metodi API in questo SDK restituiscono `pplx::task<std::shared_ptr<ResponseType>>` dal C++ REST SDK. Questo ti dà flessibilità su come gestire le risposte API.

### Chiamate sincrone con `.get()`

Usa `.get()` per bloccare il thread chiamante finché la richiesta non viene completata e ottenere il risultato in modo sincrono:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Chiamare .get() per bloccare e ottenere il risultato in modo sincrono
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
).get();  // Blocca finché la richiesta HTTP non viene completata

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Chiamate asincrone con `.then()`

Usa `.then()` per l'esecuzione asincrona non bloccante con callback:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Usa .then() per l'esecuzione asincrona basata su callback
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Questo viene eseguito in modo asincrono quando la richiesta è completata
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// L'esecuzione continua immediatamente senza bloccare
std::cout << "Request sent, continuing..." << std::endl;
```

### Scegliere tra sincrono e asincrono

La scelta dipende dal tuo ambiente di runtime e dall'architettura dell'applicazione:

**`.get()` (Bloccante sincrono)**
- Blocca il thread chiamante finché la richiesta HTTP non è completata
- Flusso di codice più semplice, più facile da comprendere
- Adatto per thread di lavoro dedicati, elaborazione batch o strumenti da riga di comando
- **Non adatto** per loop di eventi, thread GUI o server a thread singolo

**`.then()` (Asincrono non bloccante)**
- Ritorna immediatamente, il callback viene eseguito quando la richiesta è completata
- Non blocca il thread chiamante
- Necessario per architetture event-driven, applicazioni GUI o loop di eventi a thread singolo
- Permette di concatenare più operazioni
- Flusso di controllo più complesso

La suite di test dello SDK utilizza esclusivamente `.get()`, ma ciò è appropriato per l'ambiente di test dove il blocco è accettabile.