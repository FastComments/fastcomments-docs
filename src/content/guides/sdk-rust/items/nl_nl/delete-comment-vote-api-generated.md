## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| vote_id | String | Ja |  |
| url_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| edit_key | String | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'delete_comment_vote Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_vote() -> Result<VoteDeleteResponse, Error> {
    let params: DeleteCommentVoteParams = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-8f3a2b9e".to_string(),
        vote_id: "vote-7d124f".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "web-1234".to_string(),
        edit_key: Some("edit-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };

    let response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---