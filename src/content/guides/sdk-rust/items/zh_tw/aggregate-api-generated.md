透過分組（若提供 groupBy）並套用多個運算來彙總文件。支援不同的運算（例如 sum、countDistinct、avg 等）。

## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| aggregation_request | models::AggregationRequest | 是 |  |
| parent_tenant_id | String | 否 |  |
| include_stats | bool | 否 |  |

## 回應

回傳: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---