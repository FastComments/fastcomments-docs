## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetNotificationCountOptions& | Yes |  |

## 响应

返回：[`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## 示例

[inline-code-attrs-start title = 'getNotificationCount 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
GetNotificationCountOptions options;
options.filter = boost::optional<utility::string_t>(U("unread"));
options.maxCount = boost::optional<int>(10);
api->getNotificationCount(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetNotificationCountResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception& ex) {
        }
    });
[inline-code-end]