All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Llamadas sincrónicas con `.get()`

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Los parámetros requeridos son posicionales; los opcionales se colocan en la estructura de opciones
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Bloquea hasta que la solicitud HTTP se complete

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Llamadas asíncronas con `.then()`

Use `.then()` for non-blocking asynchronous execution with callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Los parámetros requeridos son posicionales; los opcionales se colocan en la estructura de opciones
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Esto se ejecuta de forma asíncrona cuando la solicitud se completa
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl; // La ejecución continúa inmediatamente sin bloqueo
```

### Elegir entre síncrono y asíncrono

The choice depends on your runtime environment and application architecture:

**`.get()` (Bloqueo síncrono)**
- Bloquea el hilo que realiza la llamada hasta que la solicitud HTTP se complete
- Flujo de código más simple, más fácil de razonar
- Adecuado para hilos de trabajo dedicados, procesamiento por lotes o herramientas de línea de comandos
- **No adecuado** para bucles de eventos, hilos GUI o servidores monohilo

**`.then()` (Asíncrono sin bloqueo)**
- Devuelve inmediatamente, el callback se ejecuta cuando la solicitud se completa
- No bloquea el hilo que realiza la llamada
- Requerido para arquitecturas dirigidas por eventos, aplicaciones GUI o bucles de eventos monohilo
- Permite encadenar múltiples operaciones
- Flujo de control más complejo

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.