## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład unPinComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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