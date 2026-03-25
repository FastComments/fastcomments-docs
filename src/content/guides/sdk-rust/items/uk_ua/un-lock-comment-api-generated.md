## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| broadcast_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`LockComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/lock_comment_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'un_lock_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnLockCommentParams = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-987654321".to_string(),
        broadcast_id: "news/world-update-2026-04-25".to_string(),
        sso: Some("sso-token-abcdef123456".to_string()),
    };
    let response: LockComment200Response = un_lock_comment(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---