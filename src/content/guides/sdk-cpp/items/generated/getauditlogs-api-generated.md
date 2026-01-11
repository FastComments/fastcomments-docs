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
boost::optional<double> limit = 50.0;
boost::optional<double> skip = 0.0;
boost::optional<SORT_DIR> order = SORT_DIR::DESC;
boost::optional<double> after = 1630454400.0;
boost::optional<double> before = 1633046400.0;
auto result_holder = std::make_shared<GetAuditLogs_200_response>();

api->getAuditLogs(tenantId, limit, skip, order, after, before)
    .then([result_holder](pplx::task<std::shared_ptr<GetAuditLogs_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) *result_holder = *resp;
        } catch (...) {
        }
    });
[inline-code-end]
