## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| commentData | CommentData | 是 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SaveCommentsResponseWithPresence.h)

## 示例

[inline-code-attrs-start title = 'createCommentPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t urlId = utility::string_t("/articles/2026/new-feature");
utility::string_t broadcastId = utility::string_t("broadcast-789");
auto commentDataPtr = std::make_shared<CommentData>();
commentDataPtr->content = utility::string_t("Great article! Thanks for sharing.");
commentDataPtr->authorEmail = utility::string_t("reader@example.com");
boost::optional<utility::string_t> sessionId = boost::optional<utility::string_t>(utility::string_t("sess-456"));
boost::optional<utility::string_t> sso;
api->createCommentPublic(tenantId, urlId, broadcastId, *commentDataPtr, sessionId, sso)
    .then([](std::shared_ptr<SaveCommentsResponseWithPresence> resp){
        (void)resp;
    });
[inline-code-end]

---