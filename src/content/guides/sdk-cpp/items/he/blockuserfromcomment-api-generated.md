## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| blockFromCommentParams | BlockFromCommentParams | כן |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |

## תגובה

מחזיר: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-blockUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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