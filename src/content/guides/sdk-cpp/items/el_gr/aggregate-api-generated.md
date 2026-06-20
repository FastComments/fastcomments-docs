---
Συγκεντρώνει έγγραφα ομαδοποιώντας τα (αν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διάφορες λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| aggregationRequest | AggregationRequest | Ναι |  |
| parentTenantId | string | Όχι |  |
| includeStats | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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