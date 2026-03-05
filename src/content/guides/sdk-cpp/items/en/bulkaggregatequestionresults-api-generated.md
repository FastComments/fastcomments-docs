## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Yes |  |
| forceRecalculate | bool | No |  |

## Response

Returns: [`BulkAggregateQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResults_200_response.h)

## Example

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
BulkAggregateQuestionResultsRequest request;
boost::optional<bool> forceRecalculate = boost::optional<bool>(true);
api->bulkAggregateQuestionResults(tenantId, request, forceRecalculate)
.then([](pplx::task<std::shared_ptr<BulkAggregateQuestionResults_200_response>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<BulkAggregateQuestionResults_200_response>();
    } catch(const std::exception&) {
    }
});
[inline-code-end]
