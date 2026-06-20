---
## Parameters

| Ім'я | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| comments_by_ids_params | models::CommentsByIdsParams | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад post_comments_by_ids'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let comments_by_ids = models::CommentsByIdsParams {
    ids: vec!["cmt-87a1".to_string(), "cmt-42b0".to_string()],
    tenant: "acme-corp-tenant".to_string(),
    site: "news/article".to_string(),
};

let params = PostCommentsByIdsParams {
    comments_by_ids_params: comments_by_ids,
    sso: Some("sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
};

let response: ModerationApiChildCommentsResponse = post_comments_by_ids(&configuration, params).await?;
[inline-code-end]

---