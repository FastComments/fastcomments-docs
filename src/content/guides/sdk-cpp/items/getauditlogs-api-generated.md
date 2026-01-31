## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| limit | double | No |  |
| skip | double | No |  |
| order | SORT_DIR | No |  |
| after | double | No |  |
| before | double | No |  |

## Response

Returns: [`GetAuditLogs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetAuditLogs_200_response.h)

## Example

[inline-code-attrs-start title = 'getAuditLogs Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> limit = 50.0;
boost::optional<double> skip = 0.0;
boost::optional<SORT_DIR> order = SORT_DIR::DESC;
boost::optional<double> after = boost::none;
boost::optional<double> before = boost::none;

api->getAuditLogs(tenantId, limit, skip, order, after, before)
.then([](pplx::task<std::shared_ptr<GetAuditLogs_200_response>> task) {
    try {
        auto resp = task.get();
        auto ack = std::make_shared<utility::string_t>(U("audit logs fetched"));
    } catch (const std::exception &e) {
    }
});
[inline-code-end]
