通过对文档进行分组（如果提供了 groupBy）并应用多个操作来聚合文档。支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| aggregationRequest | AggregationRequest | 是 |  |
| parentTenantId | string | 否 |  |
| includeStats | boolean | 否 |  |

## 响应

返回：[`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---