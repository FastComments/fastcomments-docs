## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_ids | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/checked_comments_for_blocked_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад checked_comments_for_blocked'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_checked_comments() -> Result<CheckedCommentsForBlocked200Response, Error> {
    let params: CheckedCommentsForBlockedParams = CheckedCommentsForBlockedParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_ids: "article-98765:12345,article-98765:67890".to_string(),
        sso: Some("sso-uid-4821".to_string()),
    };
    let response: CheckedCommentsForBlocked200Response = checked_comments_for_blocked(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]