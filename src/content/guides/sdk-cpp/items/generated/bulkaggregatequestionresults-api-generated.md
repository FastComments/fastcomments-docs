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
auto req = std::make_shared<BulkAggregateQuestionResultsRequest>();
boost::optional<bool> forceRecalculate(true);
api->bulkAggregateQuestionResults(tenantId, req, forceRecalculate)
    .then([](pplx::task<std::shared_ptr<BulkAggregateQuestionResults_200_response>> t){
        try {
            auto resp = t.get();
            (void)resp;
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
