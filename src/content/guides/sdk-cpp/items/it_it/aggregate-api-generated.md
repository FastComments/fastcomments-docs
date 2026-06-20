---
Aggrega i documenti raggruppandoli (se groupBy viene fornito) e applicando più operazioni. Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| aggregationRequest | AggregationRequest | Sì |  |
| parentTenantId | string | No |  |
| includeStats | bool | No |  |

## Risposta

Restituisce: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregazione'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
AggregationRequest aggregationRequest;
boost::optional<utility::string_t> parentTenant = boost::optional<utility::string_t>(utility::conversions::to_string_t("parent-tenant-456"));
boost::optional<bool> includeStats = boost::optional<bool>(true);
api->aggregate(tenantId, aggregationRequest, parentTenant, includeStats)
    .then([](pplx::task<std::shared_ptr<AggregateResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto resultCopy = std::make_shared<AggregateResponse>(*resp);
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---