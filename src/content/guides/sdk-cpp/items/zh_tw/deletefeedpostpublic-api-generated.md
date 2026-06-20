## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## 範例

[inline-code-attrs-start title = 'deleteFeedPostPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("feed-post-456");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->deleteFeedPostPublic(tenantId, postId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<DeleteFeedPostPublicResponse>> task){
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<DeleteFeedPostPublicResponse>();
    } catch (...) {
    }
}).wait();
[inline-code-end]

---