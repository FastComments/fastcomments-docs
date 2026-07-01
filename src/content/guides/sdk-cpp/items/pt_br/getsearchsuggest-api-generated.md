## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |
| options | const GetSearchSuggestOptions& | Sim |  |

## Resposta

Retorna: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## Exemplo

[inline-code-attrs-start title = 'getSearchSuggest Exemplo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetSearchSuggestOptions opts;
opts.query = U("search term");
opts.limit = boost::optional<int>(5);
opts.includeInactive = boost::optional<bool>(false);
api->getSearchSuggest(tenantId, opts).then([](pplx::task<std::shared_ptr<ModerationSuggestResponse>> t){
    try{
        auto resp = t.get();
    }catch(const std::exception&){ }
}).wait();
[inline-code-end]