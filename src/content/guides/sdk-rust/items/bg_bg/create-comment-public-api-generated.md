## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| broadcast_id | String | Да |  |
| comment_data | models::CommentData | Да |  |
| session_id | String | Не |  |
| sso | String | Не |  |

## Отговор

Връща: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за create_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateCommentPublicParams = CreateCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/politics/2026-election-coverage".to_string(),
        broadcast_id: "live-coverage-2026-03-25".to_string(),
        comment_data: models::CommentData {
            content: "Insightful piece — appreciate the depth of reporting.".to_string(),
            author_name: Some("Jane Doe".to_string()),
            author_email: Some("jane.doe@acme.com".to_string()),
            is_anonymous: Some(false),
            parent_id: None,
            metadata: None,
        },
        session_id: Some("sess_6f7e8d9c".to_string()),
        sso: Some("sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
    };
    let resp: CreateCommentPublic200Response = create_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---