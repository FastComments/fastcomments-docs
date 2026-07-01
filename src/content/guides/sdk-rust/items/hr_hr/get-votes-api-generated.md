## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Odgovor

Vraća: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_votes Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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