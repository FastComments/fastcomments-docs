## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| error_id | String | Да |  |

## Отговор

Връща: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_email_template_render_error Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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