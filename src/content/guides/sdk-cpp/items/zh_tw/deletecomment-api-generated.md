---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| contextUserId | string | 否 |  |
| isLive | bool | 否 |  |

## 回應

回傳：[`DeleteCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentResult.h)

## 範例

[inline-code-attrs-start title = 'deleteComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
boost::optional<utility::string_t> contextUserId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<bool> isLive = boost::optional<bool>(true);

api->deleteComment(tenantId, commentId, contextUserId, isLive)
    .then([](std::shared_ptr<DeleteCommentResult> result){
        auto res = result ? result : std::make_shared<DeleteCommentResult>();
        std::cout << "DeleteCommentResult ptr=" << static_cast<const void*>(res.get()) << std::endl;
    });
[inline-code-end]

---