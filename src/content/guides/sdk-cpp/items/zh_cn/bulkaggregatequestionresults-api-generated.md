## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 是 |  |
| forceRecalculate | bool | 否 |  |

## 响应

返回: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResultsResponse.h)

## 示例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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