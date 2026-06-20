## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| skip | f64 | 否 |  |

## 响应

返回：[`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## 示例

[inline-code-attrs-start title = 'get_email_template_render_errors 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetEmailTemplateRenderErrorsParams = GetEmailTemplateRenderErrorsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "welcome-email-v2".to_string(),
    skip: Some(10.0),
};
let response: GetEmailTemplateRenderErrorsResponse = get_email_template_render_errors(&configuration, params).await?;
[inline-code-end]

---