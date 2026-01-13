## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de delete_question_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteQuestionConfigParams = DeleteQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-comments-config-2025".to_string(),
        force: Some(true),
    };
    let response: FlagCommentPublic200Response = delete_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---