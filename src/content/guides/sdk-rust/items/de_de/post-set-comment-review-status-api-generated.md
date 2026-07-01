## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| reviewed | bool | Nein |  |
| broadcast_id | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'post_set_comment_review_status Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_review_status(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostSetCommentReviewStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-98765".to_string(),
        reviewed: Some(true),
        broadcast_id: Some("broadcast-2023-summer".to_string()),
        sso: Some("sso-user-42".to_string()),
    };
    post_set_comment_review_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---