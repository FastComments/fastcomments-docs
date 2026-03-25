## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| edit_key | String | Nie |  |

## Odpowiedź

Zwraca: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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