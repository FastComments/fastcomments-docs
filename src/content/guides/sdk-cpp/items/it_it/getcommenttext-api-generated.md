## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| options | const GetCommentTextOptions& | Sì |  |

## Risposta

Restituisce: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIGetCommentTextResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
auto options = std::make_shared<GetCommentTextOptions>();
options->language = boost::optional<utility::string_t>(U("en"));
options->includeDeleted = boost::optional<bool>(false);
api->getCommentText(tenantId, commentId, *options).then([](pplx::task<std::shared_ptr<PublicAPIGetCommentTextResponse>> task){
    try{
        auto response = task.get();
        (void)response;
    }catch(...){}
});
[inline-code-end]