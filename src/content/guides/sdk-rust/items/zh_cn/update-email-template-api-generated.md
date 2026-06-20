---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_email_template_body | models::UpdateEmailTemplateBody | 是 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'update_email_template 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params: UpdateEmailTemplateParams = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "weekly-newsletter".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            name: Some("Weekly Newsletter".to_string()),
            subject: Some("Your Weekly Acme Updates".to_string()),
            html: Some("<h1>Acme News</h1><p>Latest product and engineering updates.</p>".to_string()),
            plain_text: Some("Acme News - Latest product and engineering updates.".to_string()),
            enabled: Some(true),
            sender_name: Some("Acme Team".to_string()),
            sender_email: Some("newsletter@acme.com".to_string()),
            locale: Some("en-US".to_string()),
        },
    };
    update_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---