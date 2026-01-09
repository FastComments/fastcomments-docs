## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| contextUserId | string | Hayır |  |
| isLive | bool | Hayır |  |

## Yanıt

Döndürür: [`DeleteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteComment_200_response.h)

## Örnek

[inline-code-attrs-start title = 'deleteComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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