## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| reactBodyParams | ReactBodyParams | 是 |  |
| isUndo | bool | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostResponse.h)

## 示例

[inline-code-attrs-start title = 'reactFeedPostPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("feed-post-456");
ReactBodyParams reactBodyParams;
reactBodyParams.reaction = U("like");
boost::optional<bool> isUndo = false;
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-01"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso)
.then([](std::shared_ptr<ReactFeedPostResponse> resp) {
    if (resp) {
        auto resultCopy = std::make_shared<ReactFeedPostResponse>(*resp);
    }
});
[inline-code-end]

---