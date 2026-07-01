## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|------------|
| tenantId | string | Sim |  |
| options | const GetSearchSitesOptions& | Sim |  |

## Resposta

Retorna: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getSearchSites'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetSearchSitesOptions options;
options.query = boost::make_optional(utility::string_t(U("example query")));
options.limit = boost::make_optional(25);

api->getSearchSites(tenantId, options)
   .then([](pplx::task<std::shared_ptr<ModerationSiteSearchResponse>> t) {
       try {
           auto response = t.get();
           auto respPtr = std::make_shared<ModerationSiteSearchResponse>(*response);
           for (const auto& site : respPtr->sites) {
               // lógica de processamento
           }
       } catch (const std::exception&) {
           // tratamento de erro
       }
   });
[inline-code-end]