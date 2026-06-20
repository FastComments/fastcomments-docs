---
## Parâmetros

| Name | Type | Obrigatório | Descrição |
|------|------|------------|-----------|
| comment_id | String | Sim |  |
| vote_id | String | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de delete_moderation_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModerationVoteParams = DeleteModerationVoteParams {
        comment_id: "news/article-2026-06-19-12345".to_string(),
        vote_id: "vote-9a7c3b1d".to_string(),
        sso: Some("user-9876@acme-corp".to_string()),
    };
    let response: VoteDeleteResponse = delete_moderation_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---