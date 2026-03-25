## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_email_template'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let template_id: String = "welcome-new-subscriber".to_string();
    let params: GetEmailTemplateParams = GetEmailTemplateParams {
        tenant_id,
        id: template_id,
        locale: Some("en-US".to_string()),
    };
    let template: GetEmailTemplate200Response = get_email_template(&configuration, params).await?;
    let _ = template;
    Ok(())
}
[inline-code-end]