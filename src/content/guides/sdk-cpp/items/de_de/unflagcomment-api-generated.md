## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Antwort

Gibt zurück: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für unFlagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-7890");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
api->unFlagComment(tenantId, commentId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<FlagCommentResponse>> t) {
    try {
        auto resp = t.get();
        auto fallback = std::make_shared<FlagCommentResponse>();
        if (!resp) resp = fallback;
        (void)resp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]

---