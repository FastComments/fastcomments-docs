Todos os métodos da API neste SDK retornam `pplx::task<std::shared_ptr<ResponseType>>` do C++ REST SDK. Isso lhe dá flexibilidade em como lidar com as respostas da API.

### Chamadas Síncronas com `.get()`

Use `.get()` para bloquear a thread chamadora até que a requisição seja concluída e obter o resultado de forma síncrona:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Chama .get() para bloquear e obter o resultado de forma síncrona
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
).get();  // Bloqueia até que a requisição HTTP seja concluída

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Chamadas Assíncronas com `.then()`

Use `.then()` para execução assíncrona não bloqueante com callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Use .then() para execução assíncrona baseada em callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Isso é executado de forma assíncrona quando a requisição for concluída
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// A execução continua imediatamente sem bloquear
std::cout << "Request sent, continuing..." << std::endl;
```

### Escolhendo entre Síncrono e Assíncrono

A escolha depende do seu ambiente de execução e da arquitetura da aplicação:

**`.get()` (Synchronous blocking)**
- Bloqueia a thread chamadora até que a requisição HTTP seja concluída
- Fluxo de código mais simples, mais fácil de entender
- Adequado para threads de trabalho dedicadas, processamento em lote ou ferramentas de linha de comando
- **Não adequado** para loops de eventos, threads de GUI ou servidores de thread única

**`.then()` (Asynchronous non-blocking)**
- Retorna imediatamente, o callback é executado quando a requisição é concluída
- Não bloqueia a thread chamadora
- Necessário para arquiteturas orientadas a eventos, aplicações GUI ou loops de eventos de thread única
- Permite encadear múltiplas operações
- Fluxo de controle mais complexo

A suíte de testes do SDK usa `.get()` exclusivamente, mas isso é apropriado para o ambiente de testes, onde o bloqueio é aceitável.