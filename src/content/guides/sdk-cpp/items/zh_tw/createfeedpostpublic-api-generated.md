---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createFeedPostParams | CreateFeedPostParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## 範例

[inline-code-attrs-start title = 'createFeedPostPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto params = std::make_shared<CreateFeedPostParams>();
params->title = U("New feature release");
params->content = U("We launched the new comment moderation feature today.");
params->authorEmail = U("alice@example.com");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-456"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("alice@example.com"));
api->createFeedPostPublic(tenantId, *params, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            utility::string_t postId = resp->postId;
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---