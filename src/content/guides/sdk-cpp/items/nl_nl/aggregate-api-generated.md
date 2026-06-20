Agregeert documenten door ze te groeperen (als groupBy is opgegeven) en meerdere bewerkingen toe te passen.
Verschillende bewerkingen (bijv. sum, countDistinct, avg, enz.) worden ondersteund.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Ja |  |
| parentTenantId | string | Nee |  |
| includeStats | bool | Nee |  |

## Response

Retourneert: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van aggregate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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