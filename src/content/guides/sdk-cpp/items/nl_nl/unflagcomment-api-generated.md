## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Respons

Retourneert: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'unFlagComment Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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