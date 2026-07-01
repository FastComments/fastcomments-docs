## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_ids | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/check_blocked_comments_response.rs)

## Приклад

[inline-code-attrs-start title = 'checked_comments_for_blocked Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = CheckedCommentsForBlockedParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_ids: "cmt-001,cmt-002".to_string(),
        sso: Some("user@example.com".to_string()),
    };
    let _response: CheckBlockedCommentsResponse = checked_comments_for_blocked(&config, params).await?;
    Ok(())
}
[inline-code-end]