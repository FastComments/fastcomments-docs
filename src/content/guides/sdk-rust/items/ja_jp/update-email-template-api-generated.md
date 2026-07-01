## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| update_email_template_body | models::UpdateEmailTemplateBody | はい |  |

## 応答

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'update_email_template の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            subject: Some("Welcome to Acme Corp".to_string()),
            body_html: Some("<p>Hello, \{{user.name}}!</p>".to_string()),
            body_text: None,
        },
    };
    let _ = update_email_template(&config, params).await?;
    Ok(())
}
[inline-code-end]

---