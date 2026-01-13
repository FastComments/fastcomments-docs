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

戻り値: [`CreateCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateCommentPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'createCommentPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("/articles/2026/fastcomments-cpp-integration");
utility::string_t broadcastId = U("broadcast-001");
CommentData commentData;
commentData.content = U("Hello from the C++ SDK — great article!");
commentData.authorEmail = U("reader@example.com");
commentData.authorName = U("Jane Reader");
boost::optional<utility::string_t> sessionId = boost::optional<utility::string_t>(U("sess-9f3a2"));
boost::optional<utility::string_t> sso = boost::none;
auto task = api->createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso)
    .then([](pplx::task<std::shared_ptr<CreateCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto resultCopy = std::make_shared<CreateCommentPublic_200_response>(*resp);
            }
        } catch (const std::exception&) {}
    });
task.wait();
[inline-code-end]

---