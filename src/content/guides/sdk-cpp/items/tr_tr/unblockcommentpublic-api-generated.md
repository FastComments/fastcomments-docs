## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## Örnek

[inline-code-attrs-start title = 'unBlockCommentPublic Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-7890");
PublicBlockFromCommentParams params;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->unBlockCommentPublic(tenantId, commentId, params, sso)
.then([](std::shared_ptr<UnblockSuccess> res) {
    if (!res) res = std::make_shared<UnblockSuccess>();
    return res;
})
.then([](std::shared_ptr<UnblockSuccess> finalResult){
    (void)finalResult;
});
[inline-code-end]