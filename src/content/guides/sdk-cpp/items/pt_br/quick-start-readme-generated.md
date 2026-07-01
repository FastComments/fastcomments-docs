### Usando APIs Autenticadas (DefaultAPI)

**Importante:**
1. Você deve definir a URL base (o gerador cpp‑restsdk não a lê da especificação OpenAPI)  
2. Você deve definir sua chave de API no ApiClient antes de fazer solicitações autenticadas. Caso não o faça, as solicitações falharão com erro 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL (choose your region)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // REQUIRED: Set your API key
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
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

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Make public API calls
    return 0;
}
```

### Usando APIs de Moderação (ModerationApi)

O `ModerationApi` alimenta o painel de moderador. Cada método aceita um parâmetro `sso` para que a chamada seja executada em nome de um moderador autenticado via SSO (veja a seção SSO abaixo para como criar um token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // REQUIRED: Set the base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Pass the moderator's SSO token to authenticate the call
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Problemas Comuns

1. **Erro "URI must contain a hostname"**: Certifique‑se de chamar `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` antes de criar o ApiClient. O gerador cpp‑restsdk não lê automaticamente a URL do servidor da especificação OpenAPI.  
2. **Erro 401 "missing‑api‑key"**: Certifique‑se de chamar `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` antes de criar a instância DefaultAPI.  
3. **Classe de API incorreta**: Use `DefaultApi` para solicitações autenticadas do lado do servidor, `PublicApi` para solicitações cliente/públicas e `ModerationApi` para solicitações do painel de moderador (autenticadas com um token SSO de moderador).