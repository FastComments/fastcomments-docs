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

[inline-code-attrs-start title = 'Приклад lock_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: String::from("acme-news-tenant"),
        comment_id: String::from("cmt-20260325-789"),
        broadcast_id: String::from("live/politics-debate-2026-03-25"),
        sso: Some(String::from("sso-user-0a1b2c3d4e")),
    };
    let response: LockComment200Response = lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---