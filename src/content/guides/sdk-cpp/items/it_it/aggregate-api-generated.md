Aggrega i documenti raggruppandoli (se **groupBy** è fornito) e applicando più operazioni.  
Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| options | const AggregateOptions& | Yes |  |

## Risposta

Restituisce: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregazione'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // process response
        } catch (const std::exception& e) {
            // handle error
        }
    });
[inline-code-end]

---