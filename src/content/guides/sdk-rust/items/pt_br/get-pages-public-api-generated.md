Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.
Páginas que exigem SSO são filtradas de acordo com o acesso de grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| cursor | String | Não |  |
| limit | i32 | Não |  |
| q | String | Não |  |
| sort_by | models::PagesSortBy | Não |  |
| has_comments | bool | Não |  |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---