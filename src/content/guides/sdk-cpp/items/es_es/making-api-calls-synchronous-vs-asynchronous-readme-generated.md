Todos los métodos de la API en este SDK devuelven `pplx::task<std::shared_ptr<ResponseType>>` del C++ REST SDK. Esto te da flexibilidad en cómo manejas las respuestas de la API.

### Llamadas síncronas con `.get()`

Usa `.get()` para bloquear el hilo que llama hasta que la solicitud se complete y obtener el resultado de forma síncrona:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Los parámetros obligatorios son posicionales; los opcionales van en la estructura de opciones
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Llama a .get() para bloquear y obtener el resultado de forma síncrona
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Bloquea hasta que la solicitud HTTP se complete

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Llamadas asíncronas con `.then()`

Usa `.then()` para una ejecución asíncrona sin bloqueo con callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Los parámetros obligatorios son posicionales; los opcionales van en la estructura de opciones
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Usa .then() para una ejecución asíncrona basada en callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Esto se ejecuta de forma asíncrona cuando la solicitud se completa
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// La ejecución continúa inmediatamente sin bloquear
std::cout << "Request sent, continuing..." << std::endl;
```

### Elegir entre síncrono y asíncrono

La elección depende de tu entorno de ejecución y de la arquitectura de tu aplicación:

**`.get()` (Bloqueo síncrono)**
- Bloquea el hilo que llama hasta que la solicitud HTTP se completa
- Flujo de código más simple, más fácil de razonar
- Adecuado para hilos de trabajo dedicados, procesamiento por lotes o herramientas de línea de comandos
- **No es adecuado** para bucles de eventos, hilos de GUI o servidores de un solo hilo

**`.then()` (Asíncrono sin bloqueo)**
- Devuelve inmediatamente, el callback se ejecuta cuando la solicitud se completa
- No bloquea el hilo que llama
- Requerido para arquitecturas basadas en eventos, aplicaciones de GUI o bucles de eventos de un solo hilo
- Permite encadenar múltiples operaciones
- Flujo de control más complejo

La suite de pruebas del SDK usa `.get()` exclusivamente, pero esto es apropiado en el entorno de pruebas donde el bloqueo es aceptable.