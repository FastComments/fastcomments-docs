## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |

## 响应

返回: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_email_template_definitions 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEmailTemplateDefinitionsParams = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        locale: Some("en-US".to_string()),
        include_inactive: Some(false),
    };
    let templates: GetEmailTemplateDefinitions200Response =
        get_email_template_definitions(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---