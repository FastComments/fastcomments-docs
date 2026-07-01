## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Yes |  |
| create_api_page_data | models::CreateApiPageData | Yes |  |

## Risposta

Restituisce: [`AddPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_page_api_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio add_page'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = AddPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_page_data: models::CreateApiPageData {
            title: Some("Breaking News".to_string()),
            url: Some("/news/article".to_string()),
            ..Default::default()
        },
    };
    let _response = add_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]