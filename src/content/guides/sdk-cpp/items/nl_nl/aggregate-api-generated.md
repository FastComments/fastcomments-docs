Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| options | const AggregateOptions& | Yes |  |

## Response

Returns: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'aggregatie Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

AggregationRequest aggregationRequest;
aggregationRequest.setMetric(U("commentCount"));
aggregationRequest.setStartDate(U("2023-01-01T00:00:00Z"));
aggregationRequest.setEndDate(U("2023-12-31T23:59:59Z"));
aggregationRequest.setFilters({ U("status:approved") });

AggregateOptions options;
options.limit = boost::optional<int>(100);
options.includeMetadata = boost::optional<bool>(true);

api->aggregate(tenantId, aggregationRequest, options)
    .then([](pplx::task<std::shared_ptr<AggregateResponse>> task) {
        try {
            auto response = task.get();
            // verwerk respons
        } catch (const std::exception& e) {
            // afhandelen fout
        }
    });
[inline-code-end]