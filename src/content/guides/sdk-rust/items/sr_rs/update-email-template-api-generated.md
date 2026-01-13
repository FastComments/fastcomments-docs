## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| update_email_template_body | models::UpdateEmailTemplateBody | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'update_email_template Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateEmailTemplateParams = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-template-001".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            name: Some("Welcome Template".to_string()),
            subject: Some("Welcome to Acme News".to_string()),
            body_html: Some("<p>Hi {{user_name}}, welcome to Acme News!</p>".to_string()),
            from_address: Some("no-reply@acme-news.com".to_string()),
            reply_to: Some("support@acme-news.com".to_string()),
            enabled: Some(true),
            language: Some("en-US".to_string()),
            custom_config: Some(models::CustomConfigParameters {
                tracking_pixel_url: Some("https://acme-news.com/pixel".to_string()),
            }),
        },
    };

    let response: FlagCommentPublic200Response = update_email_template(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---