## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| contextUserId | string | Nein |  |
| isLive | bool | Nein |  |

## Antwort

Gibt zurück: [`DeleteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteComment_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'deleteComment Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> contextUserId = boost::optional<utility::string_t>(U("moderator@acme.com"));
boost::optional<bool> isLive = boost::optional<bool>(true);
api->deleteComment(tenantId, commentId, contextUserId, isLive)
.then([](pplx::task<std::shared_ptr<DeleteComment_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto processed = std::make_shared<DeleteComment_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---