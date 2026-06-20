---
## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| comment_id | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'post_flag_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PostFlagCommentParams = PostFlagCommentParams {
        comment_id: String::from("news/acme-corp/article-237/comment-8421"),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.acme-sso-payload")),
    };
    let response: ApiEmptyResponse = post_flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---