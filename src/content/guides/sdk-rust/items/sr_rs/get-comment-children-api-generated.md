## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| comment_id | String | Да |  |
| sso | String | Не |  |

## Одговор

Враћа: [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_comment_children'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_children() -> Result<ModerationApiChildCommentsResponse, Error> {
    let params: GetCommentChildrenParams = GetCommentChildrenParams {
        comment_id: "news/article-2026-06-19-cmt-42".to_string(),
        sso: Some("sso-token-user-8f3d2a".to_string()),
    };
    let children: ModerationApiChildCommentsResponse = get_comment_children(&configuration, params).await?;
    Ok(children)
}
[inline-code-end]

---