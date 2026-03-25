## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| create_email_template_body | models::CreateEmailTemplateBody | Sì |  |

## Risposta

Restituisce: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_email_template_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'create_email_template Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateEmailTemplateParams = CreateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".into(),
        create_email_template_body: models::CreateEmailTemplateBody {
            name: "Article Comment Notification".into(),
            subject: "New comment on your article".into(),
            body_html: "<p>Hi \{{recipient_name}},</p><p>\{{comment_author}} left a comment on your article \"\{{article_title}}\".</p>".into(),
            from_name: Some("Acme News".into()),
            from_email: Some("notifications@acme.example".into()),
            reply_to: Some("no-reply@acme.example".into()),
            enabled: Some(true),
            tags: Some(vec!["comments".into(), "notifications".into()]),
        },
    };
    let response: CreateEmailTemplate200Response = create_email_template(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---