## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|---------------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |

## Risposta

Restituisce: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio get_votes'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetVotesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        limit: Some(100),
    };
    let _response: GetVotesResponse = get_votes(configuration, params).await?;
    Ok(())
}
[inline-code-end]