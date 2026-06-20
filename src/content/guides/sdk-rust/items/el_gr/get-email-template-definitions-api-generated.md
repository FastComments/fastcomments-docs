---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_definitions_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_email_template_definitions'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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