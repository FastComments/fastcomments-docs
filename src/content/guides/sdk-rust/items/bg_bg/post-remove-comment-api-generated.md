## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| comment_id | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за post_remove_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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