通过对文档进行分组（如果提供了 groupBy）并应用多个操作来聚合文档。支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| aggregation_request | models::AggregationRequest | 是 |  |
| parent_tenant_id | String | 否 |  |
| include_stats | bool | 否 |  |

## 响应

返回: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---