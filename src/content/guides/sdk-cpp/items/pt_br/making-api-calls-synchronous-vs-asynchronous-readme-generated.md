---  
Todos os métodos de API neste SDK retornam `pplx::task<std::shared_ptr<ResponseType>>` da C++ REST SDK. Isso lhe oferece flexibilidade na forma como você trata as respostas da API.

### Chamadas síncronas com `.get()`

Use `.get()` para bloquear a thread chamadora até que a requisição seja concluída e obter o resultado de forma síncrona:

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

### Chamadas assíncronas com `.then()`

Use `.then()` para execução assíncrona não bloqueante com callbacks:

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

### Escolhendo entre síncrono e assíncrono

A escolha depende do seu ambiente de execução e da arquitetura da aplicação:

**`.get()` (Bloqueio síncrono)**
- Bloqueia a thread chamadora até que a requisição HTTP seja concluída
- Fluxo de código mais simples, mais fácil de compreender
- Adequado para threads de trabalho dedicadas, processamento em lote ou ferramentas de linha de comando
- **Não adequado** para loops de eventos, threads de GUI ou servidores de thread única

**`.then()` (Assíncrono não bloqueante)**
- Retorna imediatamente, o callback é executado quando a requisição é concluída
- Não bloqueia a thread chamadora
- Necessário para arquiteturas orientadas a eventos, aplicações GUI ou loops de eventos de thread única
- Permite encadear múltiplas operações
- Fluxo de controle mais complexo

A suíte de testes do SDK usa `.get()` exclusivamente, mas isso é apropriado para o ambiente de teste, onde o bloqueio é aceitável.