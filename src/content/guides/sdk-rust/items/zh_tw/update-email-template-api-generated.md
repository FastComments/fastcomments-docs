## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_email_template_body | models::UpdateEmailTemplateBody | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'update_email_template 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_email_template() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateEmailTemplateParams = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            subject: "Welcome to Acme News".to_string(),
            html_body: "<h1>Welcome, \{{user_name}}</h1><p>Thanks for joining Acme.</p>".to_string(),
            plain_body: Some("Welcome, \{{user_name}}!\nThanks for joining Acme.".to_string()),
            enabled: Some(true),
            from_name: Some("Acme Support <support@acme.com>".to_string()),
        },
    };
    let response: FlagCommentPublic200Response = update_email_template(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---