## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| broadcastId | string | はい |  |
| commentData | CommentData | はい |  |
| sessionId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SaveCommentsResponseWithPresence.h)

## 例

[inline-code-attrs-start title = 'createCommentPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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