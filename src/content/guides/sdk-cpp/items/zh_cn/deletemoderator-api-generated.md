## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| sendEmail | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'deleteModerator 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t moderatorId = U("moderator-456");
boost::optional<utility::string_t> sendEmail = boost::optional<utility::string_t>(U("notify@example.com"));
api->deleteModerator(tenantId, moderatorId, sendEmail)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<APIEmptyResponse>();
    } catch (...) {
        throw;
    }
}).wait();
[inline-code-end]

---