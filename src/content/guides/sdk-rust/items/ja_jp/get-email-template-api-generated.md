## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |

## レスポンス

戻り値: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_response.rs)

## 例

[inline-code-attrs-start title = 'get_email_template の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEmailTemplateParams = GetEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "marketing/welcome_v2".to_string(),
    };
    let template: GetEmailTemplateResponse = get_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---