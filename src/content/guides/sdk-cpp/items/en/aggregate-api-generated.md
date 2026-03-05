Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

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
AggregationRequest aggregationRequest;
utility::string_t tenantId( "my-tenant-123" );
boost::optional<utility::string_t> parentTenant( utility::string_t("global-tenant") );
boost::optional<bool> includeStats( true );
auto defaultResp = std::make_shared<AggregationResponse>();
api->aggregate(tenantId, aggregationRequest, parentTenant, includeStats)
    .then([defaultResp](pplx::task<std::shared_ptr<AggregationResponse>> task){
        try {
            auto resp = task.get();
            if (!resp) resp = defaultResp;
            (void)resp;
        } catch (...) {
            throw;
        }
    });
[inline-code-end]
