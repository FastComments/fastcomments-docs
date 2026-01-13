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
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> limit = 100.0;
boost::optional<double> skip = 0.0;
boost::optional<SORT_DIR> order = SORT_DIR::DESC;
boost::optional<double> after = 1622505600.0;
boost::optional<double> before = 1625097600.0;

api->getAuditLogs(tenantId, limit, skip, order, after, before)
.then([](pplx::task<std::shared_ptr<GetAuditLogs_200_response>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetAuditLogs_200_response>(*resp);
        std::cout << "Retrieved audit logs response at " << std::time(nullptr) << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Error fetching audit logs: " << e.what() << std::endl;
    }
});
[inline-code-end]
