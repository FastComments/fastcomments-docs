Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.  
Requer que `enableFChat` seja verdadeiro na configuração customizada resolvida para cada página.  
Páginas que requerem SSO são filtradas com base no acesso ao grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| cursor | String | Não |  |
| limit | i32 | Não |  |
| q | String | Não |  |
| sort_by | models::PagesSortBy | Não |  |
| has_comments | bool | Não |  |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]