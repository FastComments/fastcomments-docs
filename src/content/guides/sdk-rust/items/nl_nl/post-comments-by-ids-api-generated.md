## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| comments_by_ids_params | models::CommentsByIdsParams | Ja |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'post_comments_by_ids Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PostCommentsByIdsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comments_by_ids_params: models::CommentsByIdsParams {
            comment_ids: vec!["cmt123".to_string(), "cmt456".to_string()],
        },
        sso: Some("user-sso-token".to_string()),
    };
    let _response = post_comments_by_ids(&configuration, params).await?;
    Ok(())
}
[inline-code-end]