## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| skip | f64 | No |  |

## 回應

返回: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## 範例

[inline-code-attrs-start title = 'get_email_template_render_errors 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_template_errors(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetEmailTemplateRenderErrorsParams {
        tenant_id: "acme-corp".to_string(),
        id: "newsletter-welcome".to_string(),
        skip: Some(5.0),
    };
    let _response: GetEmailTemplateRenderErrorsResponse = get_email_template_render_errors(config, params).await?;
    Ok(())
}
[inline-code-end]