## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של deleteNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t countId = U("nc-00123");
boost::optional<utility::string_t> requestedBy = U("admin@company.com");

api->deleteNotificationCount(tenantId, countId)
.then([requestedBy](pplx::task<std::shared_ptr<APIEmptyResponse>> task) -> std::shared_ptr<APIEmptyResponse> {
    try {
        auto resp = task.get();
        (void)requestedBy;
        return resp ? resp : std::make_shared<APIEmptyResponse>();
    } catch (const std::exception&) {
        return std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---