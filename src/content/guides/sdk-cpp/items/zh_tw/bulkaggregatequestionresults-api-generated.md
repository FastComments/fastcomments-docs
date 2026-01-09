## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 是 |  |
| forceRecalculate | bool | 否 |  |

## 回傳

回傳：[`BulkAggregateQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResults_200_response.h)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
BulkAggregateQuestionResultsRequest request;
boost::optional<bool> forceRecalculate(true);
api->bulkAggregateQuestionResults(tenantId, request, forceRecalculate)
.then([](pplx::task<std::shared_ptr<BulkAggregateQuestionResults_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) {
            auto empty = std::make_shared<BulkAggregateQuestionResults_200_response>();
            std::cout << "No aggregated results returned\n";
        } else {
            std::cout << "Aggregated results received\n";
        }
    } catch (const std::exception& ex) {
        std::cerr << "Error fetching aggregated results: " << ex.what() << '\n';
    }
});
[inline-code-end]

---