## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| isFlagged | bool | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Primer

[inline-code-attrs-start title = 'flagCommentPublic Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-789012");
bool isFlagged = true;
boost::optional<utility::string_t> sso{ U("user@example.com") };
api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    auto respCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
    return respCopy;
});
[inline-code-end]

---