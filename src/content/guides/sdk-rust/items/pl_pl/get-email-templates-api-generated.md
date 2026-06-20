## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| skip | f64 | Nie |  |

## Odpowiedź

Zwraca: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_templates_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_email_templates'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_templates() -> Result<GetEmailTemplatesResponse, Error> {
    let params: GetEmailTemplatesParams = GetEmailTemplatesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetEmailTemplatesResponse = get_email_templates(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---