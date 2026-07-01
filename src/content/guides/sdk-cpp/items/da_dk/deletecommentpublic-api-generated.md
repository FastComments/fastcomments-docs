## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| options | const DeleteCommentPublicOptions& | Ja |  |

## Svar

Returnerer: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIDeleteCommentResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteCommentPublic Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
auto broadcastId = utility::string_t(U("broadcast-789"));
DeleteCommentPublicOptions options;
options.force = boost::optional<bool>(true);
options.reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteCommentPublic(tenantId, commentId, broadcastId, options)
    .then([](pplx::task<std::shared_ptr<PublicAPIDeleteCommentResponse>> t){
        try{
            auto resp = t.get();
        }catch(const std::exception& e){
        }
    });
[inline-code-end]