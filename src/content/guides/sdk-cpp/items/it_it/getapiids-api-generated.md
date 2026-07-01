## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | const GetApiIdsOptions& | Sì |  |

## Risposta

Restituisce: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## Esempio

[inline-code-attrs-start title = 'getApiIds Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetApiIdsOptions options;
options.limit = boost::optional<int>(100);
options.cursor = boost::optional<utility::string_t>(U("next-page-token"));
api->getApiIds(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentIdsResponse>> t){
    try{
        auto response = t.get();
        auto ids = response->commentIds;
    }catch(const std::exception&){
    }
});
[inline-code-end]