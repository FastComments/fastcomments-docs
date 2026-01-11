## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | bool | No |  |

## Response

Returns: [`AggregationResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregationResponse.h)

## Example

[inline-code-attrs-start title = 'aggregate Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto aggregationRequest = std::make_shared<AggregationRequest>();
boost::optional<utility::string_t> parentTenantId = boost::optional<utility::string_t>(U("parent-tenant-456"));
boost::optional<bool> includeStats = boost::optional<bool>(true);
api->aggregate(tenantId, *aggregationRequest, parentTenantId, includeStats)
.then([](std::shared_ptr<AggregationResponse> resp) {
    if (resp) {
        return resp;
    }
    return std::shared_ptr<AggregationResponse>();
});
[inline-code-end]
