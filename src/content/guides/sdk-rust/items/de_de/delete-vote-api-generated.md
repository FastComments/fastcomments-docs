## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| edit_key | String | Nein |  |

## Antwort

Gibt zurück: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'delete_vote Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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