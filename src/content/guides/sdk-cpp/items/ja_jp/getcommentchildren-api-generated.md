## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## 例

[inline-code-attrs-start title = 'getCommentChildren の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->getCommentChildren(tenantId, commentId, sso)
    .then([](pplx::task<std::shared_ptr<ModerationAPIChildCommentsResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---