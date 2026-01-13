## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| editKey | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`DeleteCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentPublic_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'deleteCommentPublic Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t commentId(U("cmt-456"));
utility::string_t broadcastId(U("brd-789"));
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(utility::string_t(U("editkey-abc123")));
boost::optional<utility::string_t> sso; 

api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<DeleteCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copied = std::make_shared<DeleteCommentPublic_200_response>(*resp);
            (void)copied;
        }
    } catch (const std::exception&) {
        auto fallback = std::make_shared<DeleteCommentPublic_200_response>();
        (void)fallback;
    }
});
[inline-code-end]