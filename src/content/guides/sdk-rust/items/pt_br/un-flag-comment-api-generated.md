## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| user_id | String | Não |  |
| anon_user_id | String | Não |  |

## Resposta

Retorna: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## Exemplo

[inline-code-attrs-start title = 'un_flag_comment Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        user_id: Some("user-67890".to_string()),
        anon_user_id: None,
    };
    let _response = un_flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---