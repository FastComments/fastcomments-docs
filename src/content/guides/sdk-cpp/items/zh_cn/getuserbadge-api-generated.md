## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 响应

返回: [`GetUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadge_200_response.h)

## 示例

[inline-code-attrs-start title = 'getUserBadge 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
boost::optional<utility::string_t> traceId = U("trace-456");
api->getUserBadge(tenantId, id).then([traceId](std::shared_ptr<GetUserBadge_200_response> resp) {
    if (!resp) resp = std::make_shared<GetUserBadge_200_response>();
    utility::string_t activeTrace = traceId.value_or(U(""));
    (void)activeTrace;
    return resp;
});
[inline-code-end]

---