## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| comment_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад post_remove_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_remove_comment() -> Result<PostRemoveCommentResponse, Error> {
    let params: PostRemoveCommentParams = PostRemoveCommentParams {
        comment_id: String::from("cmt-9f8b6a3"),
        sso: Some(String::from("sso-token-6f4e9a2b")),
    };
    let response: PostRemoveCommentResponse = post_remove_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---