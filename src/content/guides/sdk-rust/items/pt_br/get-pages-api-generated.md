## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |

## Resposta

Retorna: [`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_pages(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
    };
    let _response: GetPagesApiResponse = get_pages(configuration, params).await?;
    Ok(())
}
[inline-code-end]