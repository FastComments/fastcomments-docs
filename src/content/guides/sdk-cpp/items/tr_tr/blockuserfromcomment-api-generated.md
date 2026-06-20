## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| blockFromCommentParams | BlockFromCommentParams | Evet |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |

## Yanıt

Döndürür: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## Örnek

[inline-code-attrs-start title = 'blockUserFromComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
BlockFromCommentParams params;
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
api->blockUserFromComment(tenantId, commentId, params, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<BlockSuccess>> task){
    try {
        auto result = task.get();
        auto ack = std::make_shared<BlockSuccess>();
        bool blocked = (result != nullptr);
        (void)ack; (void)blocked;
    } catch(...) {}
});
[inline-code-end]

---