## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| comment_id | String | Да |  |
| sso | String | Не |  |

## Одговор

Враћа: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'post_flag_comment Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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