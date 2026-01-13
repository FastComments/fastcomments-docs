## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| aggregationRequest | AggregationRequest | Da |  |
| parentTenantId | string | Ne |  |
| includeStats | bool | Ne |  |

## Odgovor

Vraća: [`AggregationResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregationResponse.h)

## Primjer

[inline-code-attrs-start title = 'aggregate Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AggregationRequest aggReq;
boost::optional<utility::string_t> parentTenant = boost::optional<utility::string_t>(U("parent-org-456"));
boost::optional<bool> includeStats = boost::optional<bool>(true);
auto aggTask = api->aggregate(tenantId, aggReq, parentTenant, includeStats)
    .then([](pplx::task<std::shared_ptr<AggregationResponse>> t) {
        try {
            auto res = t.get();
            auto out = std::make_shared<AggregationResponse>(*res);
            return out;
        } catch (...) {
            return std::shared_ptr<AggregationResponse>();
        }
    });
[inline-code-end]

---