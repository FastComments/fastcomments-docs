## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| blockFromCommentParams | BlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respons

Retourneert: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## Voorbeeld

[inline-code-attrs-start title = 'blockUserFromComment Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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