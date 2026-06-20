## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| postId | string | はい |  |
| updateFeedPostParams | UpdateFeedPostParams | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## 例

[inline-code-attrs-start title = 'updateFeedPostPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
UpdateFeedPostParams updateFeedPostParams;
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostResponse>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto updatedCopy = std::make_shared<CreateFeedPostResponse>(*resp);
        }
    } catch (const std::exception& e) {
    }
});
[inline-code-end]

---