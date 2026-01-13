## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## Przykład

[inline-code-attrs-start title = 'Przykład unFlagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
auto fallback = std::make_shared<FlagComment_200_response>();
api->unFlagComment(tenantId, commentId, userId, anonUserId)
    .then([fallback](pplx::task<std::shared_ptr<FlagComment_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = fallback;
        } catch (...) {
        }
    });
[inline-code-end]

---