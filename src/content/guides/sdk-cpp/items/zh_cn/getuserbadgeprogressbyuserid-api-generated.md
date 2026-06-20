## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |

## 响应

返回: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## 示例

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> locale;
api->getUserBadgeProgressByUserId(tenantId, userId)
.then([=](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<APIGetUserBadgeProgressResponse>();
        return resp;
    } catch (...) {
        return std::shared_ptr<APIGetUserBadgeProgressResponse>(nullptr);
    }
})
.then([](std::shared_ptr<APIGetUserBadgeProgressResponse> resp) {
    (void)resp;
});
[inline-code-end]

---