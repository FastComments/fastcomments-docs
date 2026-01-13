Toutes les méthodes de l'API dans ce SDK retournent `pplx::task<std::shared_ptr<ResponseType>>` du C++ REST SDK. Cela vous offre de la flexibilité dans la manière de gérer les réponses de l'API.

### Appels synchrones avec `.get()`

Utilisez `.get()` pour bloquer le thread appelant jusqu'à ce que la requête se termine et récupérer le résultat de façon synchrone :

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Appelez .get() pour bloquer et récupérer le résultat de façon synchrone
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
).get();  // Bloque jusqu'à ce que la requête HTTP soit terminée

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Appels asynchrones avec `.then()`

Utilisez `.then()` pour une exécution asynchrone non bloquante avec des callbacks :

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Utilisez .then() pour une exécution asynchrone basée sur des callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Ceci s'exécute de façon asynchrone lorsque la requête est terminée
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// L'exécution continue immédiatement sans blocage
std::cout << "Request sent, continuing..." << std::endl;
```

### Choisir entre synchrone et asynchrone

Le choix dépend de votre environnement d'exécution et de l'architecture de votre application :

**`.get()` (Bloquant synchrone)**
- Bloque le thread appelant jusqu'à ce que la requête HTTP soit terminée
- Flux de code plus simple, plus facile à comprendre
- Convient aux threads de travail dédiés, au traitement par lots ou aux outils en ligne de commande
- **Non adapté** aux boucles d'événements, aux threads d'interface graphique ou aux serveurs mono-thread

**`.then()` (Asynchrone non bloquant)**
- Retourne immédiatement, le callback s'exécute lorsque la requête est terminée
- Ne bloque pas le thread appelant
- Requis pour les architectures pilotées par des événements, les applications GUI ou les boucles d'événements mono-thread
- Permet d'enchaîner plusieurs opérations
- Flux de contrôle plus complexe

La suite de tests du SDK utilise exclusivement `.get()`, mais cela convient à l'environnement de test où le blocage est acceptable.