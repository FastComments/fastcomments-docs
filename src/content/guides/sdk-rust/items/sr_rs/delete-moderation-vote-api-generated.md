## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| vote_id | String | Да |  |
| broadcast_id | String | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_moderation_vote Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteModerationVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-42".to_string(),
        vote_id: "vote-12345".to_string(),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: None,
    };
    let _response: VoteDeleteResponse = delete_moderation_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]