## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| contextUserId | string | Hayır |  |
| isLive | bool | Hayır |  |

## Response

Döndürür: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentResult.h)

## Örnek

[inline-code-attrs-start title = 'deleteComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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