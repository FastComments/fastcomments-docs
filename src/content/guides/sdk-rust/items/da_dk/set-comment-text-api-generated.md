---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Ja |  |
| edit_key | String | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`PublicApiSetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_set_comment_text_response.rs)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på set_comment_text'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_comment() -> Result<PublicApiSetCommentTextResponse, Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-72f3a9".to_string(),
        broadcast_id: "news/article/2026/06/19/product-launch".to_string(),
        comment_text_update_request: models::CommentTextUpdateRequest {
            text: "Updated: Congratulations on the launch! Clarified a few points.".to_string(),
        },
        edit_key: Some("edit-key-9f8b".to_string()),
        sso: Some("sso-token-user-abc123".to_string()),
    };
    let response = set_comment_text(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---