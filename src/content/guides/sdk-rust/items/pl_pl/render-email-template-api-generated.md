## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| render_email_template_body | models::RenderEmailTemplateBody | Tak |  |
| locale | String | Nie |  |

## Odpowiedź

Zwraca: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład render_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: RenderEmailTemplateParams = RenderEmailTemplateParams {
    tenant_id: "acme-corp-tenant".to_string(),
    render_email_template_body: models::RenderEmailTemplateBody {
        template_id: "comment-notification".to_string(),
        subject: Some("New comment on your article".to_string()),
        placeholders: std::collections::HashMap::from([
            ("article_title".to_string(), "Rust Gains Momentum in 2026".to_string()),
            ("comment_author".to_string(), "Jane Doe".to_string()),
            ("comment_snippet".to_string(), "Great insights — thanks for sharing!".to_string()),
        ]),
    },
    locale: Some("en-US".to_string()),
};
let rendered: RenderEmailTemplate200Response = render_email_template(&configuration, params).await?;
[inline-code-end]