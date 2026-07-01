聚合文档，通过分组（如果提供 groupBy）并应用多个操作。支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| aggregation_request | models::AggregationRequest | 是 |  |
| parent_tenant_id | String | 否 |  |
| include_stats | bool | 否 |  |

## 响应

返回: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## 示例

[inline-code-attrs-start title = '聚合 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let aggregation_request = models::AggregationRequest::default();
    let params = AggregateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        aggregation_request,
        parent_tenant_id: Some("global-tenant".to_string()),
        include_stats: Some(true),
    };
    let _response = aggregate(&config, params).await?;
    Ok(())
}
[inline-code-end]