## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Da |  |
| forceRecalculate | bool | Ne |  |

## Odgovor

Vraća: [`BulkAggregateQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResults_200_response.h)

## Primjer

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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