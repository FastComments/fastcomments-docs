## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |

## Respuesta

Devuelve: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_votes'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes_example() -> Result<(), Error> {
    let params: GetVotesParams = GetVotesParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/article/2026-01-12/housing-market"),
    };
    let votes: GetVotes200Response = get_votes(&configuration, params).await?;
    let _ = votes;
    Ok(())
}
[inline-code-end]

---