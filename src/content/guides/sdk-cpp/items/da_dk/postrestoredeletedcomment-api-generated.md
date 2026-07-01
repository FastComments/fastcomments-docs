## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostRestoreDeletedCommentOptions& | Yes |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'postRestoreDeletedComment Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654");
PostRestoreDeletedCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("Restoring after accidental delete"));
options.notifyUser = boost::optional<bool>(true);
api->postRestoreDeletedComment(tenantId, commentId, options)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
    });
[inline-code-end]