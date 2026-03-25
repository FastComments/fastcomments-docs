## Параметри

| Name | Type | Required | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| url_id | String | Так |  |
| broadcast_id | String | Так |  |
| vote_body_params | models::VoteBodyParams | Так |  |
| session_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад vote_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: VoteCommentParams = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "broadcast-nytimes-001".to_string(),
        vote_body_params: models::VoteBodyParams { ..Default::default() },
        session_id: Some("sess-9f8e7d".to_string()),
        sso: Some("user-42@example.com".to_string()),
    };
    let response: VoteComment200Response = vote_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---