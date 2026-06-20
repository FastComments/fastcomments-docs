---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 是 |  |
| forceRecalculate | bool | 否 |  |

## 回應

回傳: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResultsResponse.h)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
BulkAggregateQuestionResultsRequest request;
boost::optional<bool> forceRecalculate(true);
api->bulkAggregateQuestionResults(tenantId, request, forceRecalculate)
.then([](std::shared_ptr<BulkAggregateQuestionResultsResponse> resp) {
    if (resp) {
        auto respCopy = std::make_shared<BulkAggregateQuestionResultsResponse>(*resp);
        std::cout << "Aggregated question results received\n";
    } else {
        std::cout << "No aggregated results\n";
    }
}).wait();
[inline-code-end]

---