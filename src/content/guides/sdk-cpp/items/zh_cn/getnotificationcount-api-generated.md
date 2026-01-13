## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | bool | 否 |  |
| type | string | 否 |  |

## 响应

返回: [`GetNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCount_200_response.h)

## 示例

[inline-code-attrs-start title = 'getNotificationCount 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("https://www.example.com/article/456");
boost::optional<utility::string_t> fromCommentId = U("cmt-789");
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = U("reply");

api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
    .then([](pplx::task<std::shared_ptr<GetNotificationCount_200_response>> task){
        try {
            auto resp = task.get();
            auto result = resp ? resp : std::make_shared<GetNotificationCount_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---