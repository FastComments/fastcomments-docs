## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |

## 回傳

回傳: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_response.rs)

## 範例

[inline-code-attrs-start title = 'get_email_template_definitions 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_templates() -> Result<(), Error> {
    let params: GetEmailTemplateDefinitionsParams = GetEmailTemplateDefinitionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        locale: Some("en-US".to_string()),
    };
    let response: GetEmailTemplateDefinitionsResponse =
        get_email_template_definitions(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---