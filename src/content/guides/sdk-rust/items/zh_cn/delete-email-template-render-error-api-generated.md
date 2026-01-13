## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| error_id | String | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_email_template_render_error 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_email_template_render_error_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteEmailTemplateRenderErrorParams = DeleteEmailTemplateRenderErrorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "marketing/newsletter/welcome-email".to_string(),
        error_id: "render-err-2026-01-12-01".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_email_template_render_error(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---