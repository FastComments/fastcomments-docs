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
AggregationRequest aggregationRequest;
boost::optional<utility::string_t> parentTenantId(U("parent-tenant-456"));
boost::optional<bool> includeStats(true);
api->aggregate(tenantId, aggregationRequest, parentTenantId, includeStats)
.then([](pplx::task<std::shared_ptr<AggregationResponse>> t){
    try {
        std::shared_ptr<AggregationResponse> resp = t.get();
        std::shared_ptr<AggregationResponse> safeResp = resp ? resp : std::make_shared<AggregationResponse>();
        (void)safeResp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]
