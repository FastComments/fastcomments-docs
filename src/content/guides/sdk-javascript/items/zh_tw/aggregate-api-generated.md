透過分組（如果提供了 groupBy）並套用多個運算，來彙總文件。
支援不同的運算（例如 sum、countDistinct、avg 等）。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 是 |  |
| parentTenantId | string | 否 |  |
| includeStats | boolean | 否 |  |

## 回應

回傳：[`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---