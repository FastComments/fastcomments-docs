## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| broadcast_id | String | Tak |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Tak |  |
| edit_key | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia set_comment_text'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_comment_text() -> Result<(), Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026-03-25/comment-6789".to_string(),
        broadcast_id: "comments-broadcast-main".to_string(),
        comment_text_update_request: models::CommentTextUpdateRequest {
            text: "Updated comment: thank you @jane_doe — here's an update about #rustlang".to_string(),
            ..Default::default()
        },
        edit_key: Some("editkey-9f8e7d6c".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let result: SetCommentText200Response = set_comment_text(configuration, params).await?;
    println!("set_comment_text result: {:?}", result);
    Ok(())
}
[inline-code-end]

---