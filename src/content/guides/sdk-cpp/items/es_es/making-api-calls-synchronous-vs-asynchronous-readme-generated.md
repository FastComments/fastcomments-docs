Todos los métodos de la API en este SDK devuelven `pplx::task<std::shared_ptr<ResponseType>>` del C++ REST SDK. Esto le da flexibilidad en cómo maneja las respuestas de la API.

### Synchronous Calls with `.get()`

Use `.get()` para bloquear el hilo que realiza la llamada hasta que la solicitud se complete y recuperar el resultado de forma sincrónica:

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
).get();  // Bloquea hasta que la solicitud HTTP se complete

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchronous Calls with `.then()`

Use `.then()` para ejecución asíncrona no bloqueante con callbacks:

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

### Choosing Between Synchronous and Asynchronous

La elección depende de su entorno de ejecución y de la arquitectura de la aplicación:

**`.get()` (Synchronous blocking)**
- Bloquea el hilo que llama hasta que la solicitud HTTP se complete
- Flujo de código más simple, más fácil de razonar
- Adecuado para hilos trabajadores dedicados, procesamiento por lotes o herramientas de línea de comandos
- **No es adecuado** para bucles de eventos, hilos de GUI o servidores de un solo hilo

**`.then()` (Asynchronous non-blocking)**
- Devuelve inmediatamente, el callback se ejecuta cuando la solicitud se completa
- No bloquea el hilo que llama
- Requerido para arquitecturas dirigidas por eventos, aplicaciones GUI o bucles de eventos de un solo hilo
- Permite encadenar múltiples operaciones
- Flujo de control más complejo

El conjunto de pruebas del SDK usa `.get()` exclusivamente, pero esto es apropiado para el entorno de pruebas donde el bloqueo es aceptable.