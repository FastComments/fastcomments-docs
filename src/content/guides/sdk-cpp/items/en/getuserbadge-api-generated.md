## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
