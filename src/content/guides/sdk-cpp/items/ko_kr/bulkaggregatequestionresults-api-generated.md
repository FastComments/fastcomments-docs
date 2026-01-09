## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 예 |  |
| forceRecalculate | bool | 아니오 |  |

## 응답

반환: [`BulkAggregateQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResults_200_response.h)

## 예제

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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