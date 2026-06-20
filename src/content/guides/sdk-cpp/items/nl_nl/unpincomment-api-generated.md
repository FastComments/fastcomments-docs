## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'unPinComment Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
utility::string_t broadcastId = U("broadcast-42");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->unPinComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<ChangeCommentPinStatusResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<ChangeCommentPinStatusResponse>();
        std::cout << "Unpin operation completed\n";
    } catch (const std::exception& e) {
        std::cerr << "Unpin failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---