---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| edit_key | String | No |  |

## Risposta

Restituisce: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_vote_example() -> Result<DeleteCommentVote200Response, Error> {
    let params: DeleteVoteParams = DeleteVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-98765".to_string(),
        edit_key: Some("edit-4f2b9c".to_string()),
    };
    let response: DeleteCommentVote200Response = delete_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---