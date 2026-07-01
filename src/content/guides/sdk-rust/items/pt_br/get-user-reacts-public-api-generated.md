## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| post_ids | Vec<String> | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/user_reacts_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_user_reacts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_reacts() -> Result<(), Error> {
    let params = GetUserReactsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: Some(vec![
            "news/article-123".to_string(),
            "blog/post-456".to_string(),
        ]),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = get_user_reacts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]