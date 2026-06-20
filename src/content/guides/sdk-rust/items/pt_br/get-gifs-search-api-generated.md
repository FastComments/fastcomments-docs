## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| search | String | Sim |  |
| locale | String | Não |  |
| rating | String | Não |  |
| page | f64 | Não |  |

## Resposta

Retorna: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_gifs_search_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_gifs_search'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_gif_search() -> Result<(), Error> {
    let params: GetGifsSearchParams = GetGifsSearchParams {
        tenant_id: "acme-corp-tenant".to_string(),
        search: "breaking news".to_string(),
        locale: Some("en-US".to_string()),
        rating: Some("pg-13".to_string()),
        page: Some(1.0),
    };
    let response: GetGifsSearchResponse = get_gifs_search(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---