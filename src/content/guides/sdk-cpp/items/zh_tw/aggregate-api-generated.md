聚合文件，透過分群（若提供 groupBy）並套用多個運算。支援不同的運算（例如 sum、countDistinct、avg 等）。

## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 是 |  |
| parentTenantId | string | 否 |  |
| includeStats | bool | 否 |  |

## 回應

回傳: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## 範例

[inline-code-attrs-start title = 'aggregate 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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