## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| vote_id | String | Yes |  |
| url_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Yanıt

Döndürür: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_comment_vote Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        vote_id: "vote-67890".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-abc".to_string(),
        edit_key: Some("edit-key-xyz".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]