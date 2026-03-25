## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример delete_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_template() -> Result<(), Error> {
    let params: DeleteEmailTemplateParams = DeleteEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
    };
    let confirm_deletion: Option<bool> = Some(true);
    let response: FlagCommentPublic200Response = delete_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---