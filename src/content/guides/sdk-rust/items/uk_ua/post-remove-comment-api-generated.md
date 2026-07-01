## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| broadcast_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_api_response.rs)

## Приклад

[inline-code-attrs-start title = 'post_remove_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn remove_comment_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = PostRemoveCommentParams {
        tenant_id: "acme-corp".into(),
        comment_id: "news/article/42".into(),
        broadcast_id: Some("live-event-99".into()),
        sso: Some("sso-user-abc".into()),
    };
    let _response = post_remove_comment(config, params).await?;
    Ok(())
}
[inline-code-end]

---