## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Sì |  |
| forceRecalculate | bool | No |  |

## Risposta

Restituisce: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResultsResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di bulkAggregateQuestionResults'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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