## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|---------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| options | const PostFlagCommentOptions& | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio postFlagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
PostFlagCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.reportedBy = boost::optional<utility::string_t>(U("moderator@example.com"));

api->postFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        (void)resp;
    })
    .then([](pplx::task<void> t){
        try{
            t.get();
        }catch(const std::exception&){
        }
    });
[inline-code-end]