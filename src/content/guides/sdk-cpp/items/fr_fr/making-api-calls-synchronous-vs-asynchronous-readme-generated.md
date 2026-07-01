Toutes les méthodes d'API de ce SDK renvoient `pplx::task<std::shared_ptr<ResponseType>>` du C++ REST SDK. Cela vous offre de la flexibilité dans la façon dont vous gérez les réponses d'API.

### Appels synchrones avec `.get()`

Utilisez `.get()` pour bloquer le thread appelant jusqu'à ce que la requête se termine et récupérer le résultat de façon synchrone :

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

### Appels asynchrones avec `.then()`

Utilisez `.then()` pour une exécution asynchrone non bloquante avec des rappels :

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

### Choisir entre synchronisé et asynchrone

Le choix dépend de votre environnement d'exécution et de l'architecture de votre application :

**`.get()` (Blocage synchrone)**
- Bloque le thread appelant jusqu'à ce que la requête HTTP se termine
- Flux de code plus simple, plus facile à raisonner
- Adapté aux threads de travail dédiés, au traitement par lots ou aux outils en ligne de commande
- **Non adapté** aux boucles d'événements, aux threads d'interface graphique ou aux serveurs monothread

**`.then()` (Non bloquant asynchrone)**
- Renvoie immédiatement, le rappel s'exécute lorsque la requête se termine
- Ne bloque pas le thread appelant
- Nécessaire pour les architectures pilotées par événements, les applications GUI ou les boucles d'événements monothread
- Permet de chaîner plusieurs opérations
- Flux de contrôle plus complexe

La suite de tests du SDK utilise exclusivement `.get()`, ce qui est approprié pour l'environnement de test où le blocage est acceptable.