## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_config_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_question_config 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_question_config() -> Result<GetQuestionConfig200Response, Error> {
    let tenant: Option<String> = Some("acme-corp-tenant".to_string());
    let params: GetQuestionConfigParams = GetQuestionConfigParams {
        tenant_id: tenant.unwrap(),
        id: "news/article/2026/03/25/space-launch".to_string(),
    };
    let response: GetQuestionConfig200Response = get_question_config(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---