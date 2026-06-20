## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## Przykład

[inline-code-attrs-start title = 'Przykład blockFromCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-789");
PublicBlockFromCommentParams publicBlockFromCommentParams;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc"));
api->blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso)
    .then([](pplx::task<std::shared_ptr<BlockSuccess>> t){
        try {
            std::shared_ptr<BlockSuccess> res = t.get();
            auto copy = std::make_shared<BlockSuccess>(*res);
        } catch (const std::exception&) {}
    });
[inline-code-end]

---