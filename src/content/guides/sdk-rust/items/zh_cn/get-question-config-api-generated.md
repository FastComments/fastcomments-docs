---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_config_response.rs)

## 示例

[inline-code-attrs-start title = 'get_question_config 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_question_config() -> Result<GetQuestionConfigResponse, Error> {
    let configuration: configuration::Configuration = configuration::Configuration::default();
    let optional_tenant: Option<String> = Some("acme-corp-tenant".to_string());
    let tenant_id: String = optional_tenant.unwrap_or_else(|| "acme-default".to_string());
    let params = GetQuestionConfigParams {
        tenant_id,
        id: "news/article/2026-06-18".to_string(),
    };
    let response: GetQuestionConfigResponse = get_question_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---