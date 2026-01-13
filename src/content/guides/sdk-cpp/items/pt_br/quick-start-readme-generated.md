### Usando APIs Autenticadas (DefaultAPI)

**Importante:**
1. Você deve definir a URL base (o gerador cpp-restsdk não a lê da especificação OpenAPI)
2. Você deve configurar sua chave de API no ApiClient antes de fazer requisições autenticadas. Caso não o faça, as requisições falharão com um erro 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBRIGATÓRIO: Defina a URL base (escolha sua região)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OU: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // OBRIGATÓRIO: Defina sua chave de API
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Agora faça chamadas de API autenticadas
    return 0;
}
```

### Usando APIs Públicas (PublicAPI)

Endpoints públicos não requerem autenticação:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // OBRIGATÓRIO: Defina a URL base
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Faça chamadas de API públicas
    return 0;
}
```

### Problemas Comuns

1. **"URI must contain a hostname" error**: Certifique-se de chamar `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` antes de criar o ApiClient. O gerador cpp-restsdk não lê automaticamente a URL do servidor a partir da especificação OpenAPI.
2. **401 "missing-api-key" error**: Certifique-se de chamar `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` antes de criar a instância DefaultAPI.
3. **Wrong API class**: Use `DefaultAPI` para requisições autenticadas do lado do servidor, `PublicAPI` para requisições do lado do cliente/públicas.
---