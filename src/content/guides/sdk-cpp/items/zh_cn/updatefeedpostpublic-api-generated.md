## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| updateFeedPostParams | UpdateFeedPostParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## 示例

[inline-code-attrs-start title = 'updateFeedPostPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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