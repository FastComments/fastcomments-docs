## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| title | String | No |  |

## Risposta

Restituisce: `CreateV1PageReact`

## Esempio

[inline-code-attrs-start title = 'Esempio di create_v1_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<CreateV1PageReact, Error> {
    let params = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/06/launch".to_string(),
        title: Some("Acme Launch Coverage".to_string()),
    };
    let reaction: CreateV1PageReact = create_v1_page_react(&configuration, params).await?;
    Ok(reaction)
}
[inline-code-end]

---